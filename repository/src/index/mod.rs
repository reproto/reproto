mod file_index;
mod git_index;

pub use self::file_index::init_file_index;
use self::git_index::GitIndex;
use checksum::Checksum;
use core::{RpPackage, Version, VersionReq};
use git;
use objects::Objects;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use update::Update;
use url::Url;

/// Configuration file for objects backends.
pub struct IndexConfig {
    /// Root path when checking out local repositories.
    pub repo_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub version: Version,
    pub object: Checksum,
}

impl Deployment {
    pub fn new(version: Version, object: Checksum) -> Deployment {
        Deployment {
            version: version,
            object: object,
        }
    }
}

use errors::*;

pub trait Index {
    /// Resolve the given version of a package.
    fn resolve(
        &self,
        package: &RpPackage,
        version_req: Option<&VersionReq>,
    ) -> Result<Vec<Deployment>>;

    /// Get all versions available of a given package.
    ///
    /// The returned versions are sorted.
    fn all(&self, package: &RpPackage) -> Result<Vec<Deployment>>;

    fn put_version(
        &self,
        checksum: &Checksum,
        package: &RpPackage,
        version: &Version,
        force: bool,
    ) -> Result<()>;

    fn get_deployments(&self, package: &RpPackage, version: &Version) -> Result<Vec<Deployment>>;

    /// Get an objects URL as configured in the index.
    ///
    /// If relative, will cause objects to be loaded from the same repository as the index.
    fn objects_url(&self) -> Result<&str>;

    /// Load objects relative to the index repository.
    fn objects_from_index(&self, relative_path: &Path) -> Result<Box<Objects>>;

    /// Update local caches related to the index.
    fn update(&self) -> Result<Vec<Update>> {
        Ok(vec![])
    }
}

pub struct NoIndex;

impl Index for NoIndex {
    fn resolve(&self, _: &RpPackage, _: Option<&VersionReq>) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    fn all(&self, _: &RpPackage) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    fn put_version(&self, _: &Checksum, _: &RpPackage, _: &Version, _: bool) -> Result<()> {
        Err(ErrorKind::EmptyIndex.into())
    }

    fn get_deployments(&self, _: &RpPackage, _: &Version) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    /// Get an objects URL as configured in the index.
    ///
    /// If relative, will cause objects to be loaded from the same repository as the index.
    fn objects_url(&self) -> Result<&str> {
        Err(ErrorKind::EmptyIndex.into())
    }

    /// Load objects relative to the index repository.
    fn objects_from_index(&self, _: &Path) -> Result<Box<Objects>> {
        Err(ErrorKind::EmptyIndex.into())
    }
}

pub fn index_from_file(url: &Url) -> Result<Box<Index>> {
    let path = Path::new(url.path());

    if !path.is_dir() {
        return Err(format!("no such directory: {}", path.display()).into());
    }

    Ok(Box::new(file_index::FileIndex::new(&path)?))
}

pub fn index_from_git<'a, I>(config: IndexConfig, scheme: I, url: &'a Url) -> Result<Box<Index>>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut scheme = scheme.into_iter();

    let sub_scheme = scheme.next().ok_or_else(|| {
        format!("invalid scheme ({}), expected git+scheme", url.scheme())
    })?;

    let git_repo = git::setup_git_repo(&config.repo_dir, sub_scheme, url)?;
    let file_objects = file_index::FileIndex::new(git_repo.path())?;

    let git_repo = Rc::new(git_repo);
    let index = GitIndex::new(url.clone(), git_repo, file_objects);

    Ok(Box::new(index))
}

pub fn index_from_url(config: IndexConfig, url: &Url) -> Result<Box<Index>> {
    let mut scheme = url.scheme().split("+");
    let first = scheme.next().ok_or_else(
        || format!("invalid scheme: {}", url),
    )?;

    match first {
        "file" => index_from_file(url),
        "git" => index_from_git(config, scheme, url),
        scheme => Err(format!("unsupported scheme ({}): {}", scheme, url).into()),
    }
}
