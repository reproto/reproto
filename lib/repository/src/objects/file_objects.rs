//! ## Load objects through a local directory

use super::Objects;
use crate::checksum::Checksum;
use crate::hex_slice::HexSlice;
use reproto_core::errors::Result;
use reproto_core::Source;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
/// Load objects from the filesystem.
pub struct FileObjects {
    /// The root path of the filesystem storage.
    ///
    /// Objects will be fetched according to their checksum, like this using the example checksum
    /// `deadbeef`: `<path>/de/adbeef.reproto`
    path: PathBuf,
}

impl FileObjects {
    /// Create a new filesystem-based objects provider.
    ///
    /// `path` is the path to the objects storage.
    pub fn new<P: AsRef<Path> + ?Sized>(path: &P) -> FileObjects {
        FileObjects {
            path: path.as_ref().to_owned(),
        }
    }

    /// Calculate the path to the given checksum.
    pub fn get_path(&self, checksum: &Checksum) -> Result<PathBuf> {
        let path = self
            .path
            .join(format!("{}", HexSlice::new(&checksum[0..1])));
        let path = path.join(format!("{}", HexSlice::new(&checksum[1..2])));
        Ok(path.join(format!("{}.reproto", HexSlice::new(&checksum))))
    }
}

impl Objects for FileObjects {
    fn put_object(
        &mut self,
        checksum: &Checksum,
        source: &mut dyn Read,
        force: bool,
    ) -> Result<bool> {
        let target = self.get_path(checksum)?;

        // no need to write same file again
        if target.is_file() && !force {
            return Ok(false);
        }

        if let Some(parent) = target.parent() {
            if !parent.is_dir() {
                log::debug!("creating directory: {}", parent.display());
                fs::create_dir_all(parent)?;
            }
        }

        let mut tmp_target = target.clone();
        tmp_target.set_extension(".tmp");

        log::debug!("writing: {}", target.display());
        io::copy(source, &mut File::create(&tmp_target)?)?;
        fs::rename(tmp_target, target)?;
        return Ok(true);
    }

    fn get_object(&mut self, checksum: &Checksum) -> Result<Option<Source>> {
        let target = self.get_path(checksum)?;

        if target.is_file() {
            return Ok(Some(Source::from_path(target)));
        }

        Ok(None)
    }
}
