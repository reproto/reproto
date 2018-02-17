use super::GitRepo;
use core::errors::*;

/// An update callback.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Update<'a> {
    /// A git repository that needs updating.
    GitRepo(&'a GitRepo),
}

impl<'a> Update<'a> {
    /// Execute the specified update.
    pub fn update(&self) -> Result<()> {
        use self::Update::*;

        match *self {
            GitRepo(ref git_repo) => git_repo.update(),
        }
    }
}
