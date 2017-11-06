//! Functions and data-structures for loading a project manifest.
//!
//! Project manifests can be loaded as a convenient method for setting up language or
//! project-specific configuration for reproto.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate reproto_core;
extern crate serde;
extern crate semver;
extern crate relative_path;
extern crate toml;

pub mod errors;

use errors::*;
use relative_path::RelativePathBuf;
use reproto_core::{RpPackage, RpRequiredPackage, VersionReq};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Enum designating which language is being compiled.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Java,
    Js,
    Json,
    Python,
    Rust,
}

impl Language {
    pub fn parse(input: &str) -> Option<Language> {
        use self::Language::*;

        let language = match input {
            "java" => Java,
            "js" => Js,
            "json" => Json,
            "python" => Python,
            "rust" => Rust,
            _ => return None,
        };

        Some(language)
    }
}

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(default)]
    version: Option<VersionReq>,
}

pub fn parse_package(name: &str, value: toml::Value) -> Result<RpRequiredPackage> {
    use self::toml::Value::*;
    let package = RpPackage::parse(name);

    match value {
        String(version) => {
            let version_req = VersionReq::parse(version.as_str()).map_err(|e| {
                format!("bad version: {}: {}", e, version)
            })?;

            let version_req = if version_req == VersionReq::any() {
                None
            } else {
                Some(version_req)
            };

            Ok(RpRequiredPackage::new(package, version_req))
        }
        value => {
            let body: Package = value.try_into()?;
            Ok(RpRequiredPackage::new(package, body.version))
        }
    }
}

/// Parse a declaration of packages.
pub fn parse_packages(value: toml::Value) -> Result<Vec<RpRequiredPackage>> {
    let mut packages = Vec::new();
    let values = value.try_into::<HashMap<String, toml::Value>>()?;

    for (name, value) in values.into_iter() {
        packages.push(parse_package(name.as_str(), value)?);
    }

    Ok(packages)
}

/// A quick bundle of configuration that can be applied, depending on what the project looks like.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Preset {
    Maven {},
}

impl Preset {}

/// Apply the given preset to a manifest.
fn apply_preset_to(value: toml::Value, manifest: &mut Manifest, base: &Path) -> Result<()> {
    use self::Preset::*;
    use self::toml::Value::*;

    match value {
        String(name) => {
            match name.as_str() {
                "maven" => maven_apply_to(manifest, base)?,
                name => return Err(format!("unsupported preset: {}", name).into()),
            }
        }
        value => {
            let preset: Preset = value.try_into()?;

            match preset {
                Maven { .. } => maven_apply_to(manifest, base)?,
            }
        }
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

#[derive(Debug, Clone, Deserialize)]
pub struct JavaFields {}

#[derive(Debug, Clone, Deserialize)]
/// Common fields in the file manifest.
pub struct CommonFields {
    #[serde(default)]
    packages: Option<toml::Value>,
    #[serde(default)]
    modules: Vec<String>,
    #[serde(default)]
    presets: Vec<toml::Value>,
    #[serde(default)]
    paths: Vec<RelativePathBuf>,
}

/// The realized project manifest.
///
/// * All paths are absolute.
#[derive(Debug, Clone)]
pub struct Manifest {
    /// Language to build for.
    pub language: Option<Language>,
    /// Packages to build.
    pub packages: Vec<RpRequiredPackage>,
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
            language: None,
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
pub fn load_common_manifest(
    manifest: &mut Manifest,
    base: &Path,
    common: CommonFields,
) -> Result<()> {
    let packages = if let Some(packages) = common.packages {
        parse_packages(packages)?
    } else {
        vec![]
    };

    manifest.packages.extend(packages);
    manifest.modules.extend(common.modules);

    manifest.paths.extend(
        common.paths.iter().map(|r| r.to_path(&base)),
    );

    for preset in common.presets {
        apply_preset_to(preset, manifest, &base)?;
    }

    Ok(())
}

/// Read the given manifest.
///
/// Takes a path since it's used to convert declarations.
/// Returns `true` if the manifest is present, `false` otherwise.
pub fn read_manifest<P: AsRef<Path>, R: Read>(
    manifest: &mut Manifest,
    path: P,
    mut reader: R,
) -> Result<bool> {
    use self::Language::*;

    let path = path.as_ref();

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let value: toml::Value = toml::from_str(content.as_str()).map_err(|e| {
        format!("{}: bad manifest: {}", path.display(), e)
    })?;

    let language: Language = value
        .get("language")
        .ok_or_else(|| format!("{}: missing `language` key", path.display()))?
        .clone()
        .try_into()?;

    match language {
        Java => read_manifest_java(manifest, path, &value)?,
        Python => read_manifest_python(manifest, path, &value)?,
        Js => read_manifest_js(manifest, path, &value)?,
        Rust => read_manifest_rust(manifest, path, &value)?,
        Json => {}
    }

    manifest.language = Some(language);

    let parent = path.parent().ok_or_else(
        || format!("missing parent directory"),
    )?;

    let common: CommonFields = value.try_into()?;

    load_common_manifest(manifest, parent, common)?;
    return Ok(true);

    #[allow(unused)]
    fn read_manifest_java(manifest: &mut Manifest, path: &Path, value: &toml::Value) -> Result<()> {
        return Ok(());
    }

    #[allow(unused)]
    fn read_manifest_python(
        manifest: &mut Manifest,
        path: &Path,
        value: &toml::Value,
    ) -> Result<()> {
        return Ok(());
    }

    #[allow(unused)]
    fn read_manifest_js(manifest: &mut Manifest, path: &Path, value: &toml::Value) -> Result<()> {
        return Ok(());
    }

    #[allow(unused)]
    fn read_manifest_rust(manifest: &mut Manifest, path: &Path, value: &toml::Value) -> Result<()> {
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::Path;

    macro_rules! include_vec {
        ($name:expr) => {{
            let mut v = Vec::new();
            v.extend(include_bytes!($name).iter());
            v
        }}
    }

    macro_rules! include_manifest {
        ($name:expr) => {{
            let mut manifest = Manifest::new();

            read_manifest(
                &mut manifest,
                &Path::new(".").join($name),
                Cursor::new(include_vec!($name)),
            ).unwrap();

            manifest
        }}
    }

    #[test]
    pub fn test_string_preset() {
        let manifest = include_manifest!("tests/string_preset.reproto");
        assert_eq!(1, manifest.paths.len());
        assert!(manifest.output.is_some());
    }

    #[test]
    pub fn test_maven_preset() {
        let manifest = include_manifest!("tests/maven_preset.reproto");
        assert_eq!(1, manifest.paths.len());
        assert!(manifest.output.is_some());
    }

    #[test]
    pub fn test_paths() {
        let manifest = include_manifest!("tests/paths.reproto");
        assert_eq!(1, manifest.paths.len());
        assert!(manifest.output.is_none());
    }

    #[test]
    pub fn test_packages() {
        let manifest = include_manifest!("tests/packages.reproto");
        assert_eq!(1, manifest.packages.len());
        assert!(manifest.output.is_none());
    }

    #[test]
    pub fn test_packages_array() {
        let manifest = include_manifest!("tests/packages_array.reproto");
        assert_eq!(1, manifest.packages.len());
        assert!(manifest.output.is_none());
    }

    #[test]
    pub fn test_packages_table() {
        let manifest = include_manifest!("tests/packages_table.reproto");
        assert_eq!(1, manifest.packages.len());
        assert!(manifest.output.is_none());
    }
}
