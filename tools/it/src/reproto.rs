use crate::languages::{Instance, Language};
use crate::suite::Suite;
use anyhow::Result;
use anyhow::{bail, format_err};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::str;
use tokio::io;
use tokio::io::AsyncBufReadExt as _;
use tokio::process::Command;

pub async fn from_package(package: &str, debug: bool) -> Result<Reproto> {
    let mut cmd = Command::new("cargo");

    cmd.kill_on_drop(true);

    cmd.arg("+nightly");
    cmd.arg("build");
    cmd.args(&["--package", package]);
    cmd.args(&["--message-format", "json"]);

    let mut child = cmd
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format_err!("bad exit status: {}", e))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| format_err!("failed to get stdout"))?;

    let mut line = String::new();
    let mut stdout = io::BufReader::new(stdout);

    loop {
        line.clear();

        if stdout.read_line(&mut line).await? == 0 {
            break;
        }

        let doc: CargoLine = match serde_json::from_str(&line) {
            Ok(line) => line,
            Err(_) => continue,
        };

        if doc.target.kind.contains(&CargoKind::Bin) {
            let binary = doc
                .filenames
                .into_iter()
                .next()
                .ok_or_else(|| format_err!("expected one file name"))?;

            let binary = Path::new(&binary).to_path_buf();
            return Ok(Reproto::new(binary, debug));
        }
    }

    bail!("could not build binary");

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    pub enum CargoKind {
        #[serde(rename = "bin")]
        Bin,
        #[serde(rename = "lib")]
        Lib,
    }

    #[derive(Debug, Deserialize)]
    pub struct CargoTarget {
        kind: Vec<CargoKind>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CargoLine {
        filenames: Vec<String>,
        target: CargoTarget,
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    status: bool,
    stdout: Vec<String>,
    stderr: Vec<String>,
}

impl fmt::Display for CheckResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "status: {}", self.status)?;

        if self.stdout.is_empty() {
            writeln!(fmt, "stdout *empty*")?;
        } else {
            writeln!(fmt, "stdout:")?;

            for line in &self.stdout {
                writeln!(fmt, "{}", line)?;
            }
        }

        if self.stderr.is_empty() {
            writeln!(fmt, "stderr *empty*")?;
        } else {
            writeln!(fmt, "stderr:")?;

            for line in &self.stderr {
                writeln!(fmt, "{}", line)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Check<'m> {
    /// Path to build packages from.
    pub(crate) proto_path: &'m Path,
    /// Build the given packages.
    pub(crate) package: &'m str,
}

#[derive(Debug)]
pub(crate) struct Manifest<'a> {
    /// Working directory.
    pub(crate) suite: &'a Suite,
    /// Output directory.
    pub(crate) output: PathBuf,
    /// Language-specific options.
    pub(crate) language: &'a Language,
    /// Extra arguments.
    pub(crate) instance: &'a Instance,
    /// Package prefix to apply.
    pub(crate) package_prefix: Option<&'a str>,
}

/// Wrapping the reproto command invocation.
#[derive(Debug, Clone)]
pub struct Reproto {
    /// Path to binary.
    binary: PathBuf,
    /// Print debug output.
    #[allow(unused)]
    debug: bool,
}

impl Reproto {
    fn new(binary: PathBuf, debug: bool) -> Self {
        Self { binary, debug }
    }

    /// Build a reproto project.
    pub(crate) async fn build(&self, manifest: Manifest<'_>) -> Result<()> {
        if !manifest.suite.dir.is_dir() {
            bail!("No such proto directory: {}", manifest.suite.dir.display());
        }

        let mut cmd = Command::new(&self.binary);

        cmd.arg("build");

        if log::log_enabled!(log::Level::Debug) {
            cmd.arg("--debug");
        }

        cmd.args(&["--lang", &manifest.language.lang]);

        if let Some(package_prefix) = manifest.package_prefix {
            cmd.args(&["--package-prefix", package_prefix]);
        }

        let reproto_toml = manifest.suite.dir.join("reproto.toml");

        if reproto_toml.is_file() {
            cmd.args(&[
                "--manifest-path",
                reproto_toml.display().to_string().as_str(),
            ]);
        }

        // Output directory.
        cmd.args(&["-o", manifest.output.display().to_string().as_str()]);
        // Disable using local repository.
        cmd.arg("--no-repository");
        // Path to resolve packages from.
        cmd.args(&["--path", &manifest.suite.proto_path.display().to_string()]);

        for package in &manifest.suite.packages {
            cmd.args(&["--package", package.as_str()]);
        }

        cmd.args(&manifest.instance.args);

        log::trace!("reproto: {:?}", cmd);

        let output = cmd
            .output()
            .await
            .map_err(|e| format_err!("bad exit status: {}", e))?;

        let stdout = str::from_utf8(&output.stdout)?;

        if log::log_enabled!(log::Level::Trace) {
            if !stdout.is_empty() {
                log::trace!("reproto (stdout): {}", stdout);
            }
        }

        if !output.status.success() {
            let stderr = str::from_utf8(&output.stderr)?;

            bail!(
                "failed to run reproto on project: {}: {}:\nstdout: {}\nstderr: {}",
                manifest.suite.dir.display(),
                output.status,
                stdout,
                stderr,
            );
        }

        Ok(())
    }

    /// Check a reproto project.
    pub(crate) async fn check(&self, check: Check<'_>) -> Result<CheckResult> {
        if !check.proto_path.is_dir() {
            bail!("No such proto path: {}", check.proto_path.display());
        }

        let mut cmd = Command::new(&self.binary);

        if false {
            cmd.arg("--debug");
        }

        cmd.arg("check");

        // Do not use the local repository.
        cmd.arg("--no-repository");
        // Path to resolve packages from.
        cmd.arg("--path");
        cmd.arg(&check.proto_path);
        cmd.arg(check.package);

        let output = cmd
            .output()
            .await
            .map_err(|e| format_err!("failed to spawn reproto: {}", e))?;

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        let stdout = if !stdout.is_empty() {
            stdout.lines().map(|s| s.to_string()).collect()
        } else {
            vec![]
        };

        let stderr = if !stderr.is_empty() {
            stderr.lines().map(|s| s.to_string()).collect()
        } else {
            vec![]
        };

        Ok(CheckResult {
            status: output.status.success(),
            stdout,
            stderr,
        })
    }
}
