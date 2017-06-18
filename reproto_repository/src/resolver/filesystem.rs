use reproto_core::RpPackage;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use super::*;
use toml;

const METADATA: &'static str = "metadata";

pub struct Filesystem<'a> {
    path: &'a Path,
}

impl<'a> Filesystem<'a> {
    pub fn new(path: &Path) -> Filesystem {
        Filesystem { path: path }
    }
}

impl<'a> Resolver for Filesystem<'a> {
    fn resolve(&self, package: &RpPackage) -> Result<Option<Metadata>> {
        let path = package.parts.iter().fold(self.path.to_owned(), |a, b| a.join(b));
        let path = path.join(METADATA);

        if !path.is_file() {
            return Ok(None);
        }

        let mut f = File::open(&path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        Ok(Some(toml::from_str(&s)?))
    }
}
