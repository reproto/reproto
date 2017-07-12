use git::GitRepo;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use super::*;
use url::Url;

pub struct GitObjects {
    url: Url,
    git_repo: Arc<Mutex<GitRepo>>,
    file_objects: FileObjects,
}

impl GitObjects {
    pub fn new(url: Url, git_repo: Arc<Mutex<GitRepo>>, file_objects: FileObjects) -> GitObjects {
        GitObjects {
            url: url,
            git_repo: git_repo,
            file_objects: file_objects,
        }
    }
}

impl Objects for GitObjects {
    fn put_object(&self, _: &Checksum, _: &mut Read) -> Result<()> {
        Err(ErrorKind::NoPublishObjects(self.url.to_string()).into())
    }

    fn get_object(&self, checksum: &Checksum) -> Result<Option<PathBuf>> {
        self.file_objects.get_object(checksum)
    }

    fn update(&self) -> Result<()> {
        self.git_repo.lock().map_err(|_| ErrorKind::PoisonError)?.update()
    }
}
