use checksum::Checksum;
use core::{RpPackage, Version, VersionReq};
use errors::*;
use git::GitRepo;
use index::{Deployment, Index, file_index};
use objects::{FileObjects, GitObjects, Objects};
use std::path::Path;
use std::rc::Rc;
use update::Update;
use url::Url;

pub struct GitIndex {
    url: Url,
    git_repo: Rc<GitRepo>,
    file_index: file_index::FileIndex,
}

impl GitIndex {
    pub fn new(url: Url, git_repo: Rc<GitRepo>, file_index: file_index::FileIndex) -> GitIndex {
        GitIndex {
            url: url,
            git_repo: git_repo,
            file_index: file_index,
        }
    }
}

impl Index for GitIndex {
    fn resolve(
        &self,
        package: &RpPackage,
        version_req: Option<&VersionReq>,
    ) -> Result<Vec<Deployment>> {
        self.file_index.resolve(package, version_req)
    }

    fn put_version(&self, _: &Checksum, _: &RpPackage, _: &Version, _: bool) -> Result<()> {
        Err(ErrorKind::NoPublishIndex(self.url.to_string()).into())
    }

    fn get_deployments(&self, package: &RpPackage, version: &Version) -> Result<Vec<Deployment>> {
        self.file_index.get_deployments(package, version)
    }

    fn objects_url(&self) -> Result<&str> {
        self.file_index.objects_url()
    }

    fn objects_from_index(&self, relative_path: &Path) -> Result<Box<Objects>> {
        let path = self.file_index.path().join(relative_path);
        let file_objects = FileObjects::new(&path);

        let url = if let Some(path) = relative_path.to_str() {
            self.url.join(path)?
        } else {
            self.url.clone()
        };

        Ok(Box::new(
            GitObjects::new(url, self.git_repo.clone(), file_objects),
        ))
    }

    fn update(&self) -> Result<Vec<Update>> {
        Ok(vec![Update::GitRepo(&self.git_repo)])
    }
}
