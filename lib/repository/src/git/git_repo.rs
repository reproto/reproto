//! Abstraction over git repositories.
//! Uses git command available on the system to keep a repo in-sync.

use core::errors::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;

const GIT_BIN_ENV: &'static str = "REPROTO_GIT_BIN";
const FETCH_HEAD: &'static str = "FETCH_HEAD";

#[cfg(unix)]
mod sys {
    pub const DEFAULT_GIT_COMMAND: &'static str = "git";
}

#[cfg(windows)]
mod sys {
    pub const DEFAULT_GIT_COMMAND: &'static str = "git.exe";
}

use self::sys::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GitRepo {
    git_command: String,
    work_tree: PathBuf,
    git_dir: PathBuf,
    remote: Option<Url>,
    revspec: Option<String>,
}

impl GitRepo {
    pub fn with_remote<P: AsRef<Path>>(path: P, remote: Url, revspec: String) -> Result<GitRepo> {
        let path = path.as_ref();

        let git_command = find_git_command()?;

        let git_repo = GitRepo {
            git_command: git_command,
            work_tree: path.to_owned(),
            git_dir: path.join(".git"),
            remote: Some(remote),
            revspec: Some(revspec),
        };

        if !path.is_dir() {
            trace!("Initializing git repo in {}", path.display());
            fs::create_dir_all(path)?;
            git_repo.git(&["init"])?;
            git_repo.update()?;
        }

        Ok(git_repo)
    }

    /// Open the given path as an already existing git repository.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<GitRepo> {
        let path = path.as_ref();

        let git_command = find_git_command()?;

        Ok(GitRepo {
            git_command: git_command,
            work_tree: path.to_owned(),
            git_dir: path.join(".git"),
            remote: None,
            revspec: None,
        })
    }

    pub fn path(&self) -> &Path {
        self.work_tree.as_ref()
    }

    pub fn git(&self, args: &[&str]) -> Result<()> {
        let mut command = Command::new(&self.git_command);

        command
            .args(args)
            .env("GIT_DIR", &self.git_dir)
            .env("GIT_WORK_TREE", &self.work_tree);

        debug!("git: {:?}", command);

        let status = command.status()?;

        if !status.success() {
            let code = status.code().unwrap_or(-1);
            return Err(format!("git: bad exit code: {}", code).into());
        }

        Ok(())
    }

    /// Reset to the given revspec.
    pub fn reset(&self, revspec: &str) -> Result<()> {
        self.git(&["reset", "--hard", revspec])?;
        Ok(())
    }

    /// Commit the current staged changed to the repo with the given message.
    pub fn commit(&self, message: &str) -> Result<()> {
        self.git(&["commit", "-m", message])?;
        Ok(())
    }

    /// Add the give file.
    pub fn add<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let path_str = path.to_str()
            .ok_or_else(|| format!("{}: could not convert to string", path.display()))?;

        self.git(&["add", path_str])?;
        Ok(())
    }

    /// Update the repository.
    pub fn update(&self) -> Result<()> {
        let remote = match self.remote.as_ref() {
            None => return Ok(()),
            Some(remote) => remote,
        };

        let revspec = match self.revspec.as_ref().map(|s| s.as_str()) {
            None => return Ok(()),
            Some(revspec) => revspec,
        };

        info!("Updating {}", remote);
        self.git(&["fetch", remote.as_ref(), revspec])?;
        self.reset(FETCH_HEAD)?;

        Ok(())
    }
}

fn find_git_command() -> Result<String> {
    match env::var(GIT_BIN_ENV) {
        Ok(git_bin) => return Ok(git_bin.to_owned()),
        Err(_) => {}
    };

    Ok(DEFAULT_GIT_COMMAND.to_owned())
}
