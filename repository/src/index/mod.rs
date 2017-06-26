mod file_index;

pub use reproto_core::{RpPackage, Version, VersionReq};
use std::path::Path;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    pub version: Version,
    pub object: String,
}

use errors::*;

pub trait Index {
    fn resolve(&self, package: &RpPackage, version_req: &VersionReq) -> Result<Vec<Deployment>>;

    fn objects_url(&self) -> Result<Url>;
}

pub fn index_from_file(url: &Url) -> Result<Box<Index>> {
    let path = Path::new(url.path());

    if !path.is_dir() {
        return Err(format!("no such directory: {}", path.display()).into());
    }

    Ok(Box::new(file_index::FileIndex::new(&path)))
}

pub fn index_from_url(url: &Url) -> Result<Box<Index>> {
    match url.scheme() {
        "file" => index_from_file(url),
        scheme => Err(format!("unsupported scheme ({}): {}", scheme, url).into()),
    }
}
