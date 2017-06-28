use git::GitRepo;
use objects::{FileObjects, GitObjects, Objects};
use std::rc::Rc;
use super::*;

pub struct GitIndex {
    git_repo: Rc<GitRepo>,
    file_index: FileIndex,
}

impl GitIndex {
    pub fn new(git_repo: Rc<GitRepo>, file_index: FileIndex) -> GitIndex {
        GitIndex {
            git_repo: git_repo,
            file_index: file_index,
        }
    }
}

impl Index for GitIndex {
    fn resolve(&self,
               package: &RpPackage,
               version_req: Option<&VersionReq>)
               -> Result<Vec<Deployment>> {
        self.file_index.resolve(package, version_req)
    }

    fn put_version(&self,
                   checksum: &Checksum,
                   package: &RpPackage,
                   version: &Version)
                   -> Result<()> {
        self.file_index.put_version(checksum, package, version)
    }

    fn get_deployments(&self, package: &RpPackage, version: &Version) -> Result<Vec<Deployment>> {
        self.file_index.get_deployments(package, version)
    }

    fn objects_url(&self) -> Result<String> {
        self.file_index.objects_url()
    }

    fn objects_from_index(&self, relative_path: &Path) -> Result<Box<Objects>> {
        let git_repo = self.git_repo.clone();
        let path = self.file_index.path().join(relative_path);
        let file_objects = FileObjects::new(&path);
        Ok(Box::new(GitObjects::new(git_repo, file_objects)))
    }

    fn update(&self) -> Result<()> {
        self.git_repo.update()
    }
}
