use git::GitRepo;
use std::path::{Path, PathBuf};
use super::*;

pub struct GitObjects {
    git_repo: GitRepo,
    file_objects: FileObjects,
}

impl GitObjects {
    pub fn new(git_repo: GitRepo, file_objects: FileObjects) -> GitObjects {
        GitObjects {
            git_repo: git_repo,
            file_objects: file_objects,
        }
    }
}

impl Objects for GitObjects {
    fn put_object(&self, checksum: &Checksum, source: &Path) -> Result<()> {
        self.file_objects.put_object(checksum, source)?;
        Ok(())
    }

    fn get_object(&self, checksum: &Checksum) -> Result<Option<PathBuf>> {
        self.file_objects.get_object(checksum)
    }
}
