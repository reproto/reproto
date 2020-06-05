use crate::checksum::Checksum;
use crate::git::GitRepo;
use crate::index::{file_index, Deployment, Index};
use crate::objects::{FileObjects, GitObjects, Objects};
use crate::update::Update;
use core::errors::Result;
use core::{Range, RelativePath, RpPackage, Version};
use std::sync::Arc;
use url::Url;

pub struct GitIndex {
    url: Url,
    git_repo: Arc<GitRepo>,
    file_index: file_index::FileIndex,
    publishing: bool,
}

impl GitIndex {
    pub fn new(
        url: Url,
        git_repo: Arc<GitRepo>,
        file_index: file_index::FileIndex,
        publishing: bool,
    ) -> GitIndex {
        GitIndex {
            url,
            git_repo,
            file_index,
            publishing,
        }
    }
}

impl Index for GitIndex {
    fn resolve(&self, package: &RpPackage, range: &Range) -> Result<Vec<Deployment>> {
        self.file_index.resolve(package, range)
    }

    fn resolve_by_prefix(&self, package: &RpPackage) -> Result<Vec<(Deployment, RpPackage)>> {
        self.file_index.resolve_by_prefix(package)
    }

    fn all(&self, package: &RpPackage) -> Result<Vec<Deployment>> {
        self.file_index.all(package)
    }

    fn put_version(
        &self,
        checksum: &Checksum,
        package: &RpPackage,
        version: &Version,
        force: bool,
    ) -> Result<()> {
        if !self.publishing {
            return Err(format!(
                "index does not support publishing: {}",
                self.url.to_string()
            )
            .into());
        }

        self.file_index
            .put_version(checksum, package, version, force)?;

        let path = self.file_index.metadata_path(package);
        self.git_repo.add(path)?;
        self.git_repo
            .commit(&format!("publish: {} {}", package, version))?;

        Ok(())
    }

    fn get_deployments(&self, package: &RpPackage, version: &Version) -> Result<Vec<Deployment>> {
        self.file_index.get_deployments(package, version)
    }

    fn objects_url(&self) -> Result<&str> {
        self.file_index.objects_url()
    }

    fn objects_from_index(&self, relative_path: &RelativePath) -> Result<Box<dyn Objects>> {
        let path = relative_path.to_path(&self.file_index.path());
        let file_objects = FileObjects::new(&path);

        let mut url = self.url.clone();

        for c in relative_path.components() {
            url = url.join(c.as_str())?;
        }

        Ok(Box::new(GitObjects::new(
            url,
            self.git_repo.clone(),
            file_objects,
            self.publishing,
        )))
    }

    fn update(&self) -> Result<Vec<Update>> {
        Ok(vec![Update::GitRepo(&self.git_repo)])
    }
}
