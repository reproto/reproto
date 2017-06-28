use git::GitRepo;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use super::*;
use url::Url;

pub struct GitObjects {
    url: Url,
    git_repo: Rc<GitRepo>,
    file_objects: FileObjects,
}

impl GitObjects {
    pub fn new(url: Url, git_repo: Rc<GitRepo>, file_objects: FileObjects) -> GitObjects {
        GitObjects {
            url: url,
            git_repo: git_repo,
            file_objects: file_objects,
        }
    }
}

impl Objects for GitObjects {
    fn put_object(&self, _: &Checksum, _: &Path) -> Result<()> {
        Err(ErrorKind::NoPublishObjects(self.url.to_string()).into())
    }

    fn get_object(&self, checksum: &Checksum) -> Result<Option<PathBuf>> {
        self.file_objects.get_object(checksum)
    }

    fn update(&self) -> Result<()> {
        self.git_repo.update()
    }
}
