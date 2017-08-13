/// Some code copied from the Cargo Project at Commit:
/// https://github.com/rust-lang/cargo/commit/def249f9c18280d84f29fd96978389689fb61051

use errors::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;

const GIT: &'static str = "git";
const FETCH_HEAD: &'static str = "FETCH_HEAD";

pub struct GitRepo {
    work_tree: PathBuf,
    git_dir: PathBuf,
    remote: Url,
    revspec: String,
}

impl GitRepo {
    pub fn with_remote<P: AsRef<Path>>(path: P, remote: Url, revspec: String) -> Result<GitRepo> {
        let path = path.as_ref();

        let git_repo = GitRepo {
            work_tree: path.to_owned(),
            git_dir: path.join(".git"),
            remote: remote,
            revspec: revspec,
        };

        if !path.is_dir() {
            trace!("Initializing git repo in {}", path.display());
            fs::create_dir_all(path)?;
            git_repo.git(&["init"])?;
            git_repo.update()?;
        }

        Ok(git_repo)
    }

    pub fn path(&self) -> &Path {
        self.work_tree.as_ref()
    }

    pub fn git(&self, args: &[&str]) -> Result<()> {
        let mut command = Command::new(GIT);

        command.args(args).env("GIT_DIR", &self.git_dir).env(
            "GIT_WORK_TREE",
            &self.work_tree,
        );

        let status = command.status()?;

        if !status.success() {
            let code = status.code().unwrap_or(-1);
            return Err(format!("git: bad exit code: {}", code).into());
        }

        Ok(())
    }

    pub fn reset(&self, revspec: &str) -> Result<()> {
        self.git(&["reset", "--hard", revspec])?;
        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        info!("Updating {}", self.remote);
        self.git(
            &[
                "fetch",
                self.remote.to_string().as_str(),
                self.revspec.as_str(),
            ],
        )?;
        self.reset(FETCH_HEAD)
    }
}
