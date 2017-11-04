//! Functions and data-structures for loading a project manifest.
//!
//! Project manifests can be loaded as a convenient method for setting up language or
//! project-specific configuration for reproto.

use super::errors::*;
use std::path::{Path, PathBuf};

/// A quick bundle of configuration that can be applied, depending on what the project looks like.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Preset {
    Maven {},
}

impl Preset {
    /// Apply the given preset to a manifest.
    pub fn apply_to(&self, base: &Path, out: &mut Manifest) -> Result<()> {
        use self::Preset::*;

        match *self {
            Maven { .. } => maven_apply_to(base, out)?,
        }

        return Ok(());

        fn maven_apply_to(base: &Path, out: &mut Manifest) -> Result<()> {
            out.paths.push(base.join("src/main/reproto"));
            Ok(())
        }
    }

    pub fn maven() -> Preset {
        Preset::Maven {}
    }
}

/// The literal project manifest as it shows up in files.
#[derive(Debug, Clone, Deserialize)]
pub struct FileManifest {
    presets: Vec<Preset>,
    paths: Vec<String>,
}

/// The realized project manifest.
///
/// * All paths are absolute.
#[derive(Debug, Clone)]
pub struct Manifest {
    paths: Vec<PathBuf>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest { paths: vec![] }
    }
}

/// Load and apply all options to the given file manifest to build a realized manifest.
///
/// `base` is the base directory for which all paths specified in the manifest will be resolved.
pub fn load_manifest(base: &Path, file_manifest: FileManifest) -> Result<Manifest> {
    let mut out = Manifest::new();

    for p in file_manifest.paths {
        out.paths.push(base.join(p))?;
    }

    for preset in file_manifest.presets {
        preset.apply_to(base, &mut out)?;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_maven_preset() {
        let mut file_manifest = FileManifest::new();
        file_manifest.presets = vec![Preset::maven()];
    }
}
