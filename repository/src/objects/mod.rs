mod file_objects;
use errors::*;
use std::path::Path;
use url::Url;

pub trait Objects {}

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
