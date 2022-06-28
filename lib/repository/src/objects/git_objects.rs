//! ## Load objects through a local git repo

use crate::checksum::Checksum;
use crate::git::GitRepo;
use crate::objects::{FileObjects, Objects};
use crate::update::Update;
use reproto_core::errors::Result;
use reproto_core::Source;
use std::io::Read;
use std::sync::Arc;
use url::Url;

pub struct GitObjects {
    url: Url,
    git_repo: Arc<GitRepo>,
    file_objects: FileObjects,
    publishing: bool,
}

impl GitObjects {
    pub fn new(
        url: Url,
        git_repo: Arc<GitRepo>,
        file_objects: FileObjects,
        publishing: bool,
    ) -> GitObjects {
        GitObjects {
            url,
            git_repo,
            file_objects,
            publishing,
        }
    }
}

impl Objects for GitObjects {
    fn put_object(
        &mut self,
        checksum: &Checksum,
        reader: &mut dyn Read,
        force: bool,
    ) -> Result<bool> {
        if !self.publishing {
            return Err(format!("objects repo not support publishing: {}", self.url).into());
        }

        let added = self.file_objects.put_object(checksum, reader, force)?;

        if added {
            // add the newly added file.
            let path = self.file_objects.get_path(checksum)?;
            self.git_repo.add(path)?;
        }

        Ok(added)
    }

    fn get_object(&mut self, checksum: &Checksum) -> Result<Option<Source>> {
        self.file_objects.get_object(checksum)
    }

    fn update(&self) -> Result<Vec<Update>> {
        Ok(vec![Update::GitRepo(&self.git_repo)])
    }
}
