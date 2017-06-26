use git::GitRepo;
use super::*;

pub struct GitIndex {
    git_repo: GitRepo,
    file_index: FileIndex,
}

impl GitIndex {
    pub fn new(git_repo: GitRepo, file_index: FileIndex) -> GitIndex {
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

    fn objects_url(&self) -> Result<Url> {
        self.file_index.objects_url()
    }
}
