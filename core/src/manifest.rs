//! Functions and data-structures for loading a project manifest.
//!
//! Project manifests can be loaded as a convenient method for setting up language or
//! project-specific configuration for reproto.

use super::errors::*;
use relative_path::RelativePathBuf;
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
    /// Additional paths specified.
    pub paths: Vec<PathBuf>,
    /// Output directory.
    pub output: Option<PathBuf>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            packages: vec![],
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

    let base = base.canonicalize()?;

    for p in file_manifest.paths {
        manifest.paths.push(p.to_path(&base));
    }

    for preset in file_manifest.presets {
        preset.apply_to(manifest, &base)?;
    }

    Ok(())
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
