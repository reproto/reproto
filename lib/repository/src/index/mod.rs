mod file_index;
mod git_index;

pub use self::file_index::init_file_index;
use self::git_index::GitIndex;
use crate::checksum::Checksum;
use crate::core::errors::*;
use crate::core::{Range, RelativePath, RpPackage, Version};
use crate::git;
use crate::objects::Objects;
use crate::update::Update;
use std::path::{Path, PathBuf};
use std::sync::Arc;
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

pub trait Index: Send {
    /// Resolve the given version of a package.
    fn resolve(&self, package: &RpPackage, range: &Range) -> Result<Vec<Deployment>>;

    /// Resolve the given packages by prefix.
    fn resolve_by_prefix(&self, package: &RpPackage) -> Result<Vec<(Deployment, RpPackage)>>;

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
    fn objects_from_index(&self, relative_path: &RelativePath) -> Result<Box<dyn Objects>>;

    /// Update local caches related to the index.
    fn update(&self) -> Result<Vec<Update>> {
        Ok(vec![])
    }
}

pub struct NoIndex;

impl Index for NoIndex {
    fn resolve(&self, _: &RpPackage, _: &Range) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    fn resolve_by_prefix(&self, _: &RpPackage) -> Result<Vec<(Deployment, RpPackage)>> {
        Ok(vec![])
    }

    fn all(&self, _: &RpPackage) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    fn put_version(&self, _: &Checksum, _: &RpPackage, _: &Version, _: bool) -> Result<()> {
        Err("Empty Index".into())
    }

    fn get_deployments(&self, _: &RpPackage, _: &Version) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    /// Get an objects URL as configured in the index.
    ///
    /// If relative, will cause objects to be loaded from the same repository as the index.
    fn objects_url(&self) -> Result<&str> {
        Err("Empty Index".into())
    }

    /// Load objects relative to the index repository.
    fn objects_from_index(&self, _: &RelativePath) -> Result<Box<dyn Objects>> {
        Err("Empty Index".into())
    }
}

/// Setup an index for the given path.
pub fn index_from_path<P: AsRef<Path>>(path: P) -> Result<Box<dyn Index>> {
    let path = path.as_ref();

    if !path.is_dir() {
        return Err(format!("index: no such directory: {}", path.display()).into());
    }

    // looks like a git repo
    if path.join(".git").is_dir() {
        let git_repo = git::open_git_repo(path)?;
        let url = Url::from_file_path(path).map_err(|_| "failed to construct url")?;
        return open_git_index(&url, git_repo, true);
    }

    Ok(Box::new(file_index::FileIndex::new(&path)?))
}

fn open_git_index(url: &Url, git_repo: git::GitRepo, publishing: bool) -> Result<Box<dyn Index>> {
    let git_repo = Arc::new(git_repo);

    let file_objects = file_index::FileIndex::new(git_repo.path())?;
    let index = GitIndex::new(url.clone(), git_repo, file_objects, publishing);

    Ok(Box::new(index))
}

pub fn index_from_git<'a, I>(
    config: IndexConfig,
    scheme: I,
    url: &'a Url,
    publishing: bool,
) -> Result<Box<dyn Index>>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut scheme = scheme.into_iter();

    let sub_scheme = scheme
        .next()
        .ok_or_else(|| format!("bad scheme ({}), expected git+scheme", url.scheme()))?;

    let git_repo = if sub_scheme == "file" {
        let mut url = url.clone();

        url.set_scheme("file")
            .map_err(|_| "failed to set scheme to `file`")?;

        let path = url
            .to_file_path()
            .map_err(|_| format!("url is not a file path: {}", url))?;

        git::open_git_repo(path)?
    } else {
        git::setup_git_repo(&config.repo_dir, sub_scheme, url)?
    };

    open_git_index(url, git_repo, publishing)
}

pub fn index_from_url(config: IndexConfig, url: &Url, publishing: bool) -> Result<Box<dyn Index>> {
    let mut scheme = url.scheme().split("+");

    let first = scheme
        .next()
        .ok_or_else(|| format!("bad scheme: {}", url))?;

    match first {
        "file" => url
            .to_file_path()
            .map_err(|_| format!("url is not a file path: {}", url).into())
            .and_then(|path| index_from_path(&path)),
        "git" => index_from_git(config, scheme, url, publishing),
        scheme => Err(format!("bad scheme: {}", scheme).into()),
    }
    .chain_err(|| format!("loading index from URL: {}", url))
}
