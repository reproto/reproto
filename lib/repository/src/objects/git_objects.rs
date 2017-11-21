//! ## Load objects through a local git repo

use super::{FileObjects, Objects};
use checksum::Checksum;
use core::Object;
use errors::*;
use git::GitRepo;
use std::io::Read;
use std::rc::Rc;
use update::Update;
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
    fn put_object(&mut self, _: &Checksum, _: &mut Read, _: bool) -> Result<()> {
        Err(ErrorKind::NoPublishObjects(self.url.to_string()).into())
    }

    fn get_object(&mut self, checksum: &Checksum) -> Result<Option<Box<Object>>> {
        self.file_objects.get_object(checksum)
    }

    fn update(&self) -> Result<Vec<Update>> {
        Ok(vec![Update::GitRepo(&self.git_repo)])
    }
}
