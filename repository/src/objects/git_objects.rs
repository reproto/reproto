use git::GitRepo;
use object::Object;
use std::io::Read;
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
    fn put_object(&mut self, _: &Checksum, _: &mut Read) -> Result<()> {
        Err(ErrorKind::NoPublishObjects(self.url.to_string()).into())
    }

    fn get_object(&mut self, checksum: &Checksum) -> Result<Option<Box<Object>>> {
        self.file_objects.get_object(checksum)
    }

    fn update(&self) -> Result<()> {
        self.git_repo.update()
    }
}
