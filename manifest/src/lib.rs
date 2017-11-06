//! Functions and data-structures for loading a project manifest.
//!
//! Project manifests can be loaded as a convenient method for setting up language or
//! project-specific configuration for reproto.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate semver;
extern crate relative_path;
extern crate toml;

pub mod errors;

use errors::*;
use relative_path::RelativePathBuf;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// A quick bundle of configuration that can be applied, depending on what the project looks like.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Preset {
    Maven {},
}

impl Preset {
    /// Apply the given preset to a manifest.
    pub fn apply_to(&self, manifest: &mut Manifest, base: &Path) -> Result<()> {
        use self::Preset::*;

        match *self {
            Maven { .. } => maven_apply_to(manifest, base)?,
        }

        return Ok(());

        fn maven_apply_to(manifest: &mut Manifest, base: &Path) -> Result<()> {
            // default path
            manifest.paths.push(
                base.join("src").join("main").join("reproto"),
            );

            // output directory
            manifest.output = Some(base.join("target").join("generated").join("reproto").join(
                "java",
            ));

            Ok(())
        }
    }
}

/// The literal project manifest as it shows up in files.
#[derive(Debug, Clone, Deserialize)]
pub struct FileManifest {
    #[serde(default)]
    packages: Vec<String>,
    #[serde(default)]
    modules: Vec<String>,
    #[serde(default)]
    presets: Vec<Preset>,
    #[serde(default)]
    paths: Vec<RelativePathBuf>,
}

/// The realized project manifest.
///
/// * All paths are absolute.
#[derive(Debug, Clone)]
pub struct Manifest {
    /// Packages to build.
    pub packages: Vec<String>,
    /// Modules to enable.
    pub modules: Vec<String>,
    /// Additional paths specified.
    pub paths: Vec<PathBuf>,
    /// Output directory.
    pub output: Option<PathBuf>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            packages: vec![],
            modules: vec![],
            paths: vec![],
            output: None,
        }
    }
}

/// Load and apply all options to the given file manifest to build a realized manifest.
///
/// `manifest` is the manifest that will be populated.
/// `base` is the base directory for which all paths specified in the manifest will be resolved.
pub fn load_manifest(
    manifest: &mut Manifest,
    base: &Path,
    file_manifest: FileManifest,
) -> Result<()> {
    manifest.packages.extend(file_manifest.packages);
    manifest.modules.extend(file_manifest.modules);

    let base = base.canonicalize()?;

    for p in file_manifest.paths {
        manifest.paths.push(p.to_path(&base));
    }

    for preset in file_manifest.presets {
        preset.apply_to(manifest, &base)?;
    }

    Ok(())
}

/// Read the given manifest.
///
/// Takes a path since it's used to convert declarations.
/// Returns `true` if the manifest is present, `false` otherwise.
pub fn read_manifest<P: AsRef<Path>>(manifest: &mut Manifest, path: P) -> Result<bool> {
    use std::io::ErrorKind::*;

    let path = path.as_ref();

    let mut f = match File::open(path) {
        Err(e) => {
            match e.kind() {
                // ignore if it doesn't exist.
                NotFound => return Ok(false),
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
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_maven_preset() {
        let mut file_manifest = FileManifest::new();

        file_manifest.presets = vec![Preset::Maven {}];
    }
}
