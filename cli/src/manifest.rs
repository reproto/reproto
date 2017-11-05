//! Utilities for loading configuration files.

use core::{FileManifest, Manifest, load_manifest};
use errors::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

pub fn read_manifest<P: AsRef<Path>>(manifest: &mut Manifest, path: P) -> Result<()> {
    use std::io::ErrorKind::*;

    let path = path.as_ref();

    let mut f = match File::open(path) {
        Err(e) => {
            match e.kind() {
                // ignore if it doesn't exist.
                NotFound => return Ok(()),
                // return other errors.
                _ => return Err(e.into()),
            }
        }
        Ok(f) => f,
    };

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let file_manifest: FileManifest = toml::from_str(content.as_str()).map_err(|e| {
        format!("{}: bad manifest: {}", path.display(), e)
    })?;

    let parent = path.parent().ok_or_else(
        || format!("missing parent directory"),
    )?;

    load_manifest(manifest, parent, file_manifest)?;
    Ok(())
}
