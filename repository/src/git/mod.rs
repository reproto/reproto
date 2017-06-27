mod git_repo;

use errors::*;
pub use self::git_repo::GitRepo;
use std::path::Path;
use url::Url;

const DEFAULT_REMOTE_REF: &'static str = "refs/heads/master";

pub fn setup_git_repo<'a, P: AsRef<Path>>(repos: &P,
                                          scheme: &str,
                                          url: &'a Url)
                                          -> Result<GitRepo> {
    let mut remote = url.clone();
    remote.set_scheme(scheme).map_err(|_| format!("cannot set scheme for url: {}", url))?;

    let mut path = repos.as_ref().to_owned();

    if scheme == "file" {
        path = path.join("local");
    }

    if let Some(host) = remote.host() {
        path = path.join(format!("{}", host));
    }

    match remote.path() {
        "" => {}
        p => {
            path = p.split("/").skip(1).fold(path, |p, n| p.join(n));
        }
    }

    let refspec = remote.query_pairs()
        .find(|e| e.0 == "ref")
        .map(|e| e.1.into_owned());

    let refspec = refspec.or_else(|| {
        remote.query_pairs()
            .find(|e| e.0 == "branch")
            .map(|e| format!("refs/heads/{}", e.1.into_owned()))
    });

    let refspec = refspec.or_else(|| {
        remote.query_pairs()
            .find(|e| e.0 == "tag")
            .map(|e| format!("refs/tags/{}", e.1.into_owned()))
    });

    let refspec = refspec.unwrap_or_else(|| DEFAULT_REMOTE_REF.to_owned());

    let git_repo = GitRepo::with_remote(&path, remote, refspec)?;
    Ok(git_repo)
}
