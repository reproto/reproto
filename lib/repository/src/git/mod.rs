mod git_repo;

pub use self::git_repo::GitRepo;
use crate::core::errors::*;
use crate::sha256;
use std::path::Path;
use url::Url;

const DEFAULT_REMOTE_REF: &'static str = "refs/heads/master";

pub fn open_git_repo<P: AsRef<Path>>(path: P) -> Result<GitRepo> {
    GitRepo::open(path)
}

/// Open an already existing git repo.
pub fn setup_git_repo<'a, P: AsRef<Path>>(
    repos: &P,
    scheme: &str,
    url: &'a Url,
) -> Result<GitRepo> {
    let mut remote = url.clone();

    remote
        .set_scheme(scheme)
        .map_err(|_| format!("cannot set scheme for url: {}", url))?;

    let path = repos.as_ref().to_owned();

    let tail = {
        let mut tail = sha256::Sha256::new();
        tail.update(&remote.to_string().as_bytes());
        tail.finish()
    }?;

    let path = match remote.host() {
        Some(host) => path.join(format!("{}-{}", host, tail)),
        _ => path.join(format!("unknown-{}", tail)),
    };

    let refspec = remote
        .query_pairs()
        .find(|e| e.0 == "ref")
        .map(|e| e.1.into_owned());

    let refspec = refspec.or_else(|| {
        remote
            .query_pairs()
            .find(|e| e.0 == "branch")
            .map(|e| format!("refs/heads/{}", e.1.into_owned()))
    });

    let refspec = refspec.or_else(|| {
        remote
            .query_pairs()
            .find(|e| e.0 == "tag")
            .map(|e| format!("refs/tags/{}", e.1.into_owned()))
    });

    let refspec = refspec.unwrap_or_else(|| DEFAULT_REMOTE_REF.to_owned());

    let git_repo = GitRepo::with_remote(&path, remote, refspec)?;
    Ok(git_repo)
}
