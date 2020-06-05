use crate::languages::{Instance, Language, Languages};
use crate::reproto::{Manifest, Reproto};
use crate::suite::Suite;
use crate::utils;
use crate::{timed_run, Action, BoxFuture, Error, Runner, TimedRun};
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Setup projects.
pub async fn setup(
    root: &Path,
    action: Action,
    filter: impl Fn(&[&str]) -> bool,
    reproto: &Reproto,
    languages: &Languages,
    suites: &[Suite],
    runners: &mut Vec<Box<dyn Runner>>,
) -> Result<()> {
    for language in &languages.languages {
        for suite in suites {
            if !suite.supports_language(&language.name) {
                log::trace!(
                    "language `{}` not supported by suite `{}`",
                    language.name,
                    suite.name
                );
                continue;
            }

            for instance in &language.instances {
                if !filter(&["structure", &suite.name, &instance.name, &language.name]) {
                    continue;
                }

                let expected_struct = root
                    .join("structures")
                    .join(&language.name)
                    .join(&format!("{}-{}", suite.name, instance.name));

                #[cfg(feature = "migrate-structures")]
                {
                    let old_expected_struct = root
                        .join("suites")
                        .join(&suite.name)
                        .join("structures")
                        .join(&instance.name)
                        .join(match language.name.as_str() {
                            "python2" => "python",
                            other => other,
                        });

                    if old_expected_struct.is_dir() && !expected_struct.is_dir() {
                        if let Some(parent) = expected_struct.parent() {
                            fs::create_dir_all(parent).await?;
                        }

                        log::info!(
                            "migrating: {} -> {}",
                            old_expected_struct.display(),
                            expected_struct.display()
                        );
                        let mut options = fs_extra::dir::CopyOptions::new();
                        options.copy_inside = true;
                        fs_extra::dir::copy(&old_expected_struct, &expected_struct, &options)?;
                    }
                }

                let target_struct = root
                    .join("target")
                    .join("structures")
                    .join(&language.name)
                    .join(&format!("{}-{}", suite.name, instance.name));

                runners.push(Box::new(StructureRunner {
                    instance: instance.clone(),
                    action,
                    expected_struct,
                    target_struct,
                    suite: suite.clone(),
                    language: language.clone(),
                    reproto: reproto.clone(),
                }));
            }
        }
    }

    Ok(())
}

/// A runner that builds a specification and compares with a known, expected structure.
#[derive(Debug)]
struct StructureRunner {
    instance: Instance,
    /// Action to run.
    action: Action,
    /// Source directory to verify structure against.
    expected_struct: PathBuf,
    /// Target directory to build structure.
    target_struct: PathBuf,
    /// Current directory of project to build.
    suite: Suite,
    /// Language-specific options.
    language: Language,
    /// Reproto command wrapper.
    reproto: Reproto,
}

impl StructureRunner {
    fn manifest<'a>(&'a self, output: &Path) -> Manifest<'a> {
        Manifest {
            suite: &self.suite,
            output: output.to_owned(),
            language: &self.language,
            instance: &self.instance,
            package_prefix: None,
        }
    }

    async fn try_run(&self) -> Result<()> {
        match self.action {
            Action::Update => {
                if self.expected_struct.is_dir() {
                    // Remove existing expected directory.
                    fs::remove_dir_all(&self.expected_struct).await?;
                }

                fs::create_dir_all(&self.expected_struct).await?;
                self.reproto
                    .build(self.manifest(&self.expected_struct))
                    .await?;
            }
            Action::Verify => {
                // expect nothing
                if !self.expected_struct.is_dir() {
                    return Ok(());
                }

                if self.target_struct.is_dir() {
                    // Remove existing directory, and re-recreate it.
                    fs::remove_dir_all(&self.target_struct).await?;
                }

                fs::create_dir_all(&self.target_struct).await?;
                self.reproto
                    .build(self.manifest(&self.target_struct))
                    .await?;

                let mut errors = Vec::new();
                utils::diff_recursive(&self.expected_struct, &self.target_struct, &mut errors)?;

                if errors.is_empty() {
                    return Ok(());
                }

                return Err(Error::Differences {
                    from: self.expected_struct.to_owned(),
                    to: self.target_struct.to_owned(),
                    errors,
                }
                .into());
            }
        }

        Ok(())
    }
}

impl Runner for StructureRunner {
    /// Run the suite.
    fn run<'o>(&'o self) -> BoxFuture<'o, (TimedRun, Result<()>)> {
        Box::pin(async move {
            let id = format!(
                "structure {} (lang: {}, instance: {})",
                self.suite.name, self.language.name, self.instance.name,
            );

            timed_run(id, self.try_run()).await
        })
    }
}
