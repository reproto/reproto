//! Tools for working with a build manifest.

use anyhow::Result;
use relative_path::RelativePathBuf;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::str;
use tokio::time;

use crate::docker::Docker;
use crate::languages::{Instance, Language};
use crate::run;
use crate::suite::Suite;
use crate::utils;

/// Load the build.yaml from the specified path.
pub(crate) fn load_path(path: &Path) -> Result<file::BuildYaml> {
    let f = std::fs::File::open(path)?;
    let f: file::BuildYaml = serde_yaml::from_reader(f)?;
    Ok(f)
}

#[derive(Debug)]
pub(crate) struct BuildYaml {
    is_prebuilt: bool,
    /// Environment variables to add to every docker invocation.
    env: BTreeMap<String, String>,
    pub(crate) deadline: std::time::Duration,
    container: String,
    steps: Vec<Step>,
    run: Run,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum Step {
    /// Copy a file from a path, to a different path.
    Copy {
        from: RelativePathBuf,
        to: RelativePathBuf,
    },
    /// Build step using docker.
    Run { command: String },
}

#[derive(Debug)]
pub(crate) struct Run {
    command: String,
}

impl BuildYaml {
    pub(crate) async fn build<'a>(
        &self,
        foreground: bool,
        deadline: time::Instant,
        target: &Path,
        language: &Language,
        suite: &Suite,
        instance: &Instance,
    ) -> Result<run::Run> {
        let docker = Docker::new(foreground);

        let container = if self.is_prebuilt {
            format!("reproto-it-{}", language.name)
        } else {
            self.container.clone()
        };

        let name = format!(
            "reproto-it-{}-{}-{}",
            language.name, suite.name, instance.name
        );

        let path = target.join("Dockerfile");
        log::trace!(
            "{name}: building container: {path}",
            name = name,
            path = path.display()
        );
        let out = std::fs::File::create(&path)?;
        write_dockerfile(out, &container, &self.env, &self.steps)?;
        docker.build(&name, &target, deadline, "Dockerfile").await?;

        Ok(match &self.run {
            Run { command } => run::Run {
                docker,
                container: name.clone(),
                command: command.clone(),
            },
        })
    }

    /// Extract all containers used by this builder.
    pub(crate) fn containers(&self, containers: &mut HashSet<String>) {
        containers.insert(self.container.clone());
    }
}

impl Step {
    fn write_dockerfile<W>(&self, mut out: W) -> io::Result<()>
    where
        W: io::Write,
    {
        match self {
            Self::Copy { from, to } => {
                writeln!(out, "COPY {} {}", from, to)?;
            }
            Self::Run { command } => {
                writeln!(out, "RUN {}", command)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PreBuild {
    container: String,
    pub name: String,
    steps: Vec<Step>,
    env: BTreeMap<String, String>,
    deadline: std::time::Duration,
    source_languages: PathBuf,
    shared_languages: PathBuf,
}

impl PreBuild {
    /// Build the prepared container.
    pub async fn build(&self, foreground: bool) -> Result<String> {
        utils::copy_dir(&self.source_languages, &self.shared_languages)?;

        let docker = Docker::new(foreground);

        // Abide by the specified deadline specified in the build.
        let deadline = time::Instant::now() + time::Duration::from(self.deadline);

        let path = self.shared_languages.join("Dockerfile");

        log::trace!(
            "{name}: building container: {path}",
            name = self.name,
            path = path.display()
        );

        let out = std::fs::File::create(path)?;
        write_dockerfile(out, &self.container, &self.env, &self.steps)?;

        docker
            .build(&self.name, &self.shared_languages, deadline, "Dockerfile")
            .await?;

        Ok(self.name.clone())
    }
}

fn write_dockerfile<'env, E, W>(
    mut out: W,
    container: &str,
    env: E,
    steps: &[Step],
) -> io::Result<()>
where
    W: io::Write,
    E: IntoIterator<Item = (&'env String, &'env String)>,
{
    writeln!(out, "FROM {}", container)?;

    for (key, value) in env {
        writeln!(out, "ENV {} {}", key, value)?;
    }

    writeln!(out, "WORKDIR /run")?;
    writeln!(out, "COPY . /run")?;

    for step in steps {
        step.write_dockerfile(&mut out)?;
    }

    Ok(())
}

mod file {
    use crate::languages::Language;
    use anyhow::Result;
    use relative_path::RelativePathBuf;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use std::path::Path;

    fn default_deadline() -> std::time::Duration {
        std::time::Duration::from_secs(120)
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct BuildYaml {
        /// Environment variables to add to every docker invocation.
        #[serde(default)]
        pub(crate) env: BTreeMap<String, String>,
        #[serde(with = "humantime_serde", default = "default_deadline")]
        pub(crate) deadline: std::time::Duration,
        pub(crate) container: String,
        #[serde(default)]
        pub(crate) prepare: Vec<super::Step>,
        #[serde(default)]
        pub(crate) steps: Vec<Step>,
        pub(crate) run: Run,
    }

    impl BuildYaml {
        /// If this build.yaml has a prebuild step, set it up.
        pub(crate) fn prebuild(
            &self,
            source_languages: &Path,
            shared_languages: &Path,
            language: &Language,
        ) -> Option<super::PreBuild> {
            if !self.prepare.is_empty() {
                Some(super::PreBuild {
                    container: self.container.clone(),
                    name: format!("reproto-it-{}", language.name),
                    steps: self.prepare.clone(),
                    env: self.env.clone(),
                    deadline: self.deadline,
                    source_languages: source_languages.to_owned(),
                    shared_languages: shared_languages.to_owned(),
                })
            } else {
                None
            }
        }

        pub(crate) fn compile<T>(
            &self,
            hbs: &handlebars::Handlebars<'_>,
            vars: &T,
        ) -> Result<super::BuildYaml>
        where
            T: Serialize,
        {
            let mut steps = Vec::new();

            for s in &self.steps {
                steps.push(s.compile(hbs, vars)?);
            }

            Ok(super::BuildYaml {
                is_prebuilt: !self.prepare.is_empty(),
                env: self.env.clone(),
                deadline: self.deadline,
                container: self.container.clone(),
                steps,
                run: self.run.compile(hbs, vars)?,
            })
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(tag = "type", rename_all = "kebab-case")]
    pub(crate) enum Step {
        /// Copy a file from a path, to a different path.
        Copy {
            from: TemplateString,
            to: TemplateString,
        },
        /// Build step using docker.
        Run { command: TemplateString },
    }

    impl Compile<super::Step> for Step {
        fn compile<T>(&self, hbs: &handlebars::Handlebars<'_>, vars: &T) -> Result<super::Step>
        where
            T: Serialize,
        {
            Ok(match self {
                Self::Copy { from, to } => super::Step::Copy {
                    from: from.compile(hbs, vars)?,
                    to: to.compile(hbs, vars)?,
                },
                Self::Run { command } => super::Step::Run {
                    command: command.compile(hbs, vars)?,
                },
            })
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    pub(crate) struct Run {
        command: TemplateString,
    }

    impl Compile<super::Run> for Run {
        fn compile<T>(&self, hbs: &handlebars::Handlebars<'_>, vars: &T) -> Result<super::Run>
        where
            T: Serialize,
        {
            Ok(super::Run {
                command: self.command.compile(hbs, vars)?,
            })
        }
    }

    trait Compile<O> {
        fn compile<T>(&self, hbs: &handlebars::Handlebars<'_>, vars: &T) -> Result<O>
        where
            T: Serialize;
    }

    impl Compile<RelativePathBuf> for TemplateString {
        fn compile<T>(&self, hbs: &handlebars::Handlebars<'_>, vars: &T) -> Result<RelativePathBuf>
        where
            T: Serialize,
        {
            let path = hbs.render_template(&self.inner, vars)?;
            Ok(RelativePathBuf::from(path))
        }
    }

    impl Compile<String> for TemplateString {
        fn compile<T>(&self, hbs: &handlebars::Handlebars<'_>, vars: &T) -> Result<String>
        where
            T: Serialize,
        {
            Ok(hbs.render_template(&self.inner, vars)?)
        }
    }

    #[derive(Debug, Clone)]
    pub struct TemplateString {
        inner: String,
    }

    impl<'de> serde::de::Deserialize<'de> for TemplateString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let inner = String::deserialize(deserializer)?;

            Ok(TemplateString { inner })
        }
    }
}
