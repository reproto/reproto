use hex_slice::HexSlice;
use std::fs;
use std::path::{Path, PathBuf};
use super::*;

pub struct FileObjects {
    path: PathBuf,
}

impl FileObjects {
    pub fn new<P: AsRef<Path> + ?Sized>(path: &P) -> FileObjects {
        FileObjects { path: path.as_ref().to_owned() }
    }

    fn checksum_path(&self, checksum: &Checksum) -> Result<PathBuf> {
        let path = self.path.join(format!("{}", HexSlice::new(&checksum[0..1])));
        let path = path.join(format!("{}", HexSlice::new(&checksum[1..2])));
        Ok(path.join(format!("{}", HexSlice::new(&checksum))))
    }
}

impl Objects for FileObjects {
    fn put_object(&self, checksum: &Checksum, source: &Path) -> Result<()> {
        let target = self.checksum_path(checksum)?;

        // no need to write same file again
        if !target.is_file() {
            if let Some(parent) = target.parent() {
                if !parent.is_dir() {
                    debug!("creating directory: {}", parent.display());
                    fs::create_dir_all(parent)?;
                }
            }

            let mut tmp_target = target.clone();
            tmp_target.set_extension(".tmp");

            debug!("writing: {} (from: {})", target.display(), source.display());

            fs::copy(source, &tmp_target)?;
            fs::rename(tmp_target, target)?;
        }

        Ok(())
    }

    fn get_object(&self, checksum: &Checksum) -> Result<Option<PathBuf>> {
        let target = self.checksum_path(checksum)?;

        if target.is_file() {
            return Ok(Some(target));
        }

        Ok(None)
    }
}
