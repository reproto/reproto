mod file_objects;
use errors::*;
pub use sha256::Checksum;
use std::path::{Path, PathBuf};
use url::Url;

pub trait Objects {
    /// Put the given object into the database.
    /// This will cause the object denoted by the given checksum to be uploaded to the objects
    /// store.
    fn put_object(&self, checksum: &Checksum, source: &Path) -> Result<()>;

    /// Get a path to the object with the given checksum.
    /// This might cause the object to be downloaded if it's not already present in the local
    /// filesystem.
    fn get_object(&self, checksum: &Checksum) -> Result<Option<PathBuf>>;
}

pub fn objects_from_file(url: &Url) -> Result<Box<Objects>> {
    let path = Path::new(url.path());

    if !path.is_dir() {
        return Err(format!("no such directory: {}", path.display()).into());
    }

    Ok(Box::new(file_objects::FileObjects::new(&path)))
}

pub fn objects_from_url(url: &Url) -> Result<Box<Objects>> {
    match url.scheme() {
        "file" => objects_from_file(url),
        scheme => Err(format!("unsupported scheme ({}): {}", scheme, url).into()),
    }
}
