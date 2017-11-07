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
use relative_path::{RelativePath, RelativePathBuf};
use reproto_core::{RpPackage, RpRequiredPackage, Version, VersionReq};
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

            let version_req = if version_req.is_wildcard() {
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

/// Parse a single file declaration.
pub fn parse_file(base: &Path, package: &str, value: toml::Value) -> Result<ManifestFile> {
    use self::toml::Value::*;
    let package = RpPackage::parse(package);

    match value {
        String(path) => {
            let path = RelativePath::new(path.as_str()).to_path(base);

            Ok(ManifestFile {
                path: path,
                package: Some(package),
                version: None,
            })
        }
        value => {
            let body: ImManifestFile = value.try_into()?;

            Ok(ManifestFile {
                path: body.path.to_path(base),
                package: body.package,
                version: body.version,
            })
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

pub fn parse_files(base: &Path, value: toml::Value) -> Result<Vec<ManifestFile>> {
    let mut packages = Vec::new();
    let values = value.try_into::<HashMap<String, toml::Value>>()?;

    for (name, value) in values.into_iter() {
        packages.push(parse_file(base, name.as_str(), value)?);
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

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ImRepository {
    /// Skip using local repository.
    #[serde(default)]
    no_repository: bool,
    /// URL to use for index.
    #[serde(default)]
    index: Option<String>,
    /// URL to use to objects storage.
    #[serde(default)]
    objects: Option<String>,
}

/// Common fields in the file manifest.
#[derive(Debug, Clone, Deserialize)]
pub struct ImCommonFields {
    /// Version to use for unversioned specs.
    #[serde(default)]
    version: Option<Version>,
    /// Packages to build.
    #[serde(default)]
    packages: Option<toml::Value>,
    /// Files to build.
    #[serde(default)]
    files: Option<toml::Value>,
    #[serde(default)]
    modules: Vec<String>,
    #[serde(default)]
    presets: Vec<toml::Value>,
    #[serde(default)]
    paths: Vec<RelativePathBuf>,
    #[serde(default)]
    package_prefix: Option<RpPackage>,
    #[serde(default)]
    id_converter: Option<String>,
    #[serde(default)]
    repository: ImRepository,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImManifestFile {
    pub path: RelativePathBuf,
    pub package: Option<RpPackage>,
    pub version: Option<Version>,
}

#[derive(Debug, Clone)]
pub struct ManifestFile {
    pub path: PathBuf,
    pub package: Option<RpPackage>,
    pub version: Option<Version>,
}

impl ManifestFile {
    pub fn from_path(path: &Path) -> ManifestFile {
        ManifestFile {
            path: path.to_owned(),
            package: None,
            version: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Repository {
    /// Skip using local repository.
    pub no_repository: bool,
    /// URL to use for index.
    pub index: Option<String>,
    /// URL to use to objects storage.
    pub objects: Option<String>,
}

/// The realized project manifest.
///
/// * All paths are absolute.
#[derive(Debug, Clone, Default)]
pub struct Manifest {
    /// Version to use for unversioned specs.
    /// Required when publishing.
    pub version: Option<Version>,
    /// Language to build for.
    pub language: Option<Language>,
    /// Packages to build.
    pub packages: Vec<RpRequiredPackage>,
    /// Files to build.
    pub files: Vec<ManifestFile>,
    /// Modules to enable.
    pub modules: Vec<String>,
    /// Additional paths specified.
    pub paths: Vec<PathBuf>,
    /// Output directory.
    pub output: Option<PathBuf>,
    /// Package prefix to apply.
    pub package_prefix: Option<RpPackage>,
    /// Conversion strategy to use for IDs.
    pub id_converter: Option<String>,
    /// Repository configuration.
    pub repository: Repository,
}

/// Load and apply all repository-specific information.
pub fn load_repository(
    repository: &mut Repository,
    _base: &Path,
    input: ImRepository,
) -> Result<()> {
    repository.no_repository = input.no_repository;
    repository.index = input.index;
    repository.objects = input.objects;
    Ok(())
}

/// Load and apply all options to the given file manifest to build a realized manifest.
///
/// `manifest` is the manifest that will be populated.
/// `base` is the base directory for which all paths specified in the manifest will be resolved.
pub fn load_common_manifest(
    manifest: &mut Manifest,
    base: &Path,
    common: ImCommonFields,
) -> Result<()> {
    let packages = if let Some(packages) = common.packages {
        parse_packages(packages)?
    } else {
        vec![]
    };

    let files = if let Some(files) = common.files {
        parse_files(base, files)?
    } else {
        vec![]
    };

    manifest.packages.extend(packages);
    manifest.files.extend(files);
    manifest.modules.extend(common.modules);

    manifest.version = common.version;

    manifest.paths.extend(
        common.paths.iter().map(|r| r.to_path(&base)),
    );

    for preset in common.presets {
        apply_preset_to(preset, manifest, &base)?;
    }

    if let Some(package_prefix) = common.package_prefix {
        manifest.package_prefix = Some(package_prefix);
    }

    if let Some(id_converter) = common.id_converter {
        manifest.id_converter = Some(id_converter);
    }

    load_repository(&mut manifest.repository, base, common.repository)?;
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

    if let Some(language) = value.get("language") {
        let language: Language = language.clone().try_into::<Language>().map_err(|e| {
            format!("bad `language` key: {}", e)
        })?;

        match language {
            Java => read_manifest_java(manifest, path, &value)?,
            Python => read_manifest_python(manifest, path, &value)?,
            Js => read_manifest_js(manifest, path, &value)?,
            Rust => read_manifest_rust(manifest, path, &value)?,
            Json => {}
        }

        manifest.language = Some(language);
    }

    let parent = path.parent().ok_or_else(
        || format!("missing parent directory"),
    )?;

    let common: ImCommonFields = value.try_into()?;

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
            let mut manifest = Manifest::default();

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
