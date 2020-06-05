use crate::docker::Docker;
use crate::Result;
use tokio::process::Command;

#[derive(Debug)]
pub(crate) struct Run {
    pub(crate) docker: Docker,
    pub(crate) container: String,
    pub(crate) command: String,
}

impl Run {
    pub fn command(self) -> Result<Command> {
        self.docker.command(&self.container, &self.command)
    }
}
