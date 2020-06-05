use anyhow::{bail, Result};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Stdio;
use std::str;
use tokio::process::Command;
use tokio::time;

/// Install the collection of containers specified.
pub async fn install_containers<'a, I>(containers: I) -> Result<()>
where
    I: IntoIterator<Item = &'a String>,
{
    let mut to_pull = containers
        .into_iter()
        .map(|s| s.as_str())
        .collect::<BTreeSet<&str>>();

    let output = Command::new("docker")
        .args(&["image", "ls", "--format", "{{.Repository}}:{{.Tag}}"])
        .output()
        .await?;

    if !output.status.success() {
        bail!("failed to list docker images")
    }

    let stdout = str::from_utf8(&output.stdout)?;

    for image in stdout.split('\n') {
        to_pull.remove(image.trim());
    }

    if to_pull.is_empty() {
        return Ok(());
    }

    println!("pulling images: {:?}", to_pull);

    for image in to_pull {
        println!("checking image: {}", image);

        let status = Command::new("docker")
            .arg("pull")
            .arg(image)
            .status()
            .await?;

        if !status.success() {
            bail!("failed to pull image `{}`: {}", image, status);
        }
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) struct Docker {
    foreground: bool,
}

impl<'a> Docker {
    pub(crate) fn new(foreground: bool) -> Self {
        Self { foreground }
    }

    pub(crate) fn command(&self, container: &str, command: &str) -> Result<Command> {
        let mut c = Command::new("docker");
        c.kill_on_drop(true);
        c.arg("run");
        c.arg("-i");
        c.args(&["--entrypoint", "/bin/sh"]);
        c.arg(container);
        c.args(&["-c", &command]);
        Ok(c)
    }

    /// Run the given command with the specified deadline.
    pub(crate) async fn build(
        &self,
        name: &str,
        path: &Path,
        deadline: time::Instant,
        file: &str,
    ) -> Result<()> {
        let mut command = Command::new("docker");

        command.current_dir(path);
        command.kill_on_drop(true);
        command.arg("build");
        command.args(&["--tag", name]);
        command.args(&["--file", file]);
        command.arg(".");

        self.run_command_with_deadline(command, deadline).await?;
        Ok(())
    }

    async fn run_command_with_deadline(
        &self,
        mut command: Command,
        deadline: time::Instant,
    ) -> Result<()> {
        log::trace!("run: {:?}", command);

        let timeout = time::delay_until(deadline);

        if self.foreground {
            command.stdout(Stdio::inherit());
            command.stderr(Stdio::inherit());

            let status = tokio::select! {
                status = command.status() => {
                    status?
                },
                _ = timeout => {
                    bail!("command timed out");
                },
            };

            if !status.success() {
                bail!("failed to build project with docker: {}", status,);
            }
        } else {
            let output = tokio::select! {
                output = command.output() => {
                    output?
                },
                _ = timeout => {
                    bail!("command timed out");
                },
            };

            if !output.status.success() {
                let stdout = str::from_utf8(&output.stdout)?;
                let stderr = str::from_utf8(&output.stderr)?;

                bail!(
                    "failed to build project with docker: {}\nstdout: {}\nstderr: {}",
                    output.status,
                    stdout,
                    stderr,
                );
            }
        }

        Ok(())
    }
}
