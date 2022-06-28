use crate::reproto::{Check, CheckResult, Reproto};
use crate::{timed_run, Action, BoxFuture, Error, Runner, TimedRun};
use anyhow::{bail, format_err, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt as _;

pub async fn setup(
    root: &Path,
    action: Action,
    filter: impl Fn(&[&str]) -> bool,
    reproto: &Reproto,
    runners: &mut Vec<Box<dyn Runner>>,
) -> Result<()> {
    let ui = root.join("ui");

    if !ui.is_dir() {
        bail!("missing ui directory: {}", ui.display());
    }

    let checks_path = ui.join("checks");
    let proto_path = ui.join("proto");

    if !proto_path.is_dir() {
        bail!("missing proto directory: {}", proto_path.display());
    }

    for package in find_packages(&proto_path).await? {
        log::trace!("discovered: {}", package);

        if !filter(&vec!["ui", &package]) {
            continue;
        }

        if let Action::Update = action {
            if !checks_path.is_dir() {
                log::trace!("creating directory: {}", checks_path.display());
                fs::create_dir_all(&checks_path).await?;
            }
        }

        let check_path = checks_path.join(format!("{}.json", package));

        runners.push(Box::new(UiRunner {
            proto_path: proto_path.clone(),
            check_path,
            package,
            action,
            reproto: reproto.clone(),
        }));
    }

    Ok(())
}

async fn find_packages(path: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();

    let mut s = fs::read_dir(path).await?;

    while let Some(entry) = s.next_entry().await? {
        if let Ok(name) = entry.file_name().into_string() {
            if let Some(package) = name.strip_suffix(".reproto") {
                out.push(package.to_string());
            }
        }
    }

    Ok(out)
}

#[derive(Debug)]
pub struct UiRunner {
    /// Path to the proto directory.
    proto_path: PathBuf,
    /// Path to the check being compared against.
    check_path: PathBuf,
    /// Package to build.
    package: String,
    /// Action to run.
    action: Action,
    /// Reproto command wrapper.
    reproto: Reproto,
}

/// Perform a check and compare expected errors.
impl UiRunner {
    fn check<'a>(&'a self) -> Check<'a> {
        Check {
            proto_path: &self.proto_path,
            package: &self.package,
        }
    }

    async fn try_run(&self) -> Result<()> {
        match self.action {
            Action::Update => {
                let result = self.reproto.check(self.check()).await?;

                let bytes = serde_json::to_string_pretty(&result)?;

                let mut f = fs::File::create(&self.check_path).await?;
                f.write_all(bytes.as_bytes()).await?;
            }
            Action::Verify => {
                let actual = self.reproto.check(self.check()).await?;

                if !self.check_path.is_file() {
                    return Err(Error::CheckFailed {
                        expected: None,
                        actual,
                    }
                    .into());
                }

                let f = fs::read_to_string(&self.check_path).await.map_err(|e| {
                    format_err!("failed to open: {}: {}", self.check_path.display(), e)
                })?;

                let expected: CheckResult = serde_json::from_str(&f)?;

                if actual != expected {
                    bail!("ACTUAL:\n{}\nEXPECTED:\n{}", actual, expected);
                }
            }
        }

        Ok(())
    }
}

impl Runner for UiRunner {
    /// Run the suite.
    fn run<'a>(&'a self) -> BoxFuture<'a, (TimedRun, Result<()>)> {
        Box::pin(async move {
            let id = format!("ui (package: {})", self.package,);

            timed_run(id, self.try_run()).await
        })
    }
}
