//! Functions and data-structures for loading a project manifest.
//!
//! Project manifests can be loaded as a convenient method for setting up language or
//! project-specific configuration for reproto.

extern crate relative_path;
extern crate reproto_core as core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use core::{Range, RpPackage, RpRequiredPackage, Version};
use core::errors::Result;
use relative_path::{RelativePath, RelativePathBuf};
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};

/// The trait that describes the specific implementation of a given language.
pub trait Lang: Default {
    /// Module type used.
    type Module: 'static + TryFromToml;

    /// Comment the given string.
    fn comment(input: &str) -> Option<String>;
}

/// Fallback language support in case no language is specified.
#[derive(Default)]
pub struct NoLang;

pub enum NoModule {
}

impl NoModule {
    /// Indicate that the provided values are not legal through an error.
    pub fn illegal<T, V: fmt::Display>(_: &Path, id: &str, value: V) -> Result<T> {
        Err(format!("illegal module: {} => {}", id, value).into())
    }
}

impl TryFromToml for NoModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

impl Lang for NoLang {
    type Module = NoModule;

    fn comment(_input: &str) -> Option<String> {
        None
    }
}

/// Trait to convert different types.
pub trait TryFromToml
where
    Self: Sized,
{
    /// Convert from a string value.
    fn try_from_string(base: &Path, id: &str, value: String) -> Result<Self>;

    /// Convert from a TOML.
    fn try_from_value(base: &Path, id: &str, value: toml::Value) -> Result<Self>;
}

impl TryFromToml for ManifestFile {
    fn try_from_string(base: &Path, id: &str, value: String) -> Result<Self> {
        let package = RpPackage::parse(id);
        let path = RelativePath::new(value.as_str()).to_path(base);

        Ok(ManifestFile {
            path: path,
            package: Some(package),
            version: None,
        })
    }

    fn try_from_value(base: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let package = RpPackage::parse(id);
        let body: ImManifestFile = value.try_into()?;

        return Ok(ManifestFile {
            path: body.path.to_path(base),
            package: Some(package),
            version: body.version,
        });

        #[derive(Debug, Clone, Deserialize)]
        pub struct ImManifestFile {
            pub path: RelativePathBuf,
            pub version: Option<Version>,
        }
    }
}

impl TryFromToml for Publish {
    fn try_from_string(_: &Path, id: &str, value: String) -> Result<Self> {
        let package = RpPackage::parse(id);
        let version =
            Version::parse(value.as_str()).map_err(|e| format!("bad version: {}: {}", e, value))?;

        Ok(Publish {
            package: package,
            version: version,
        })
    }

    fn try_from_value(_: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let package = RpPackage::parse(id);
        let body: ImPublish = value.try_into()?;

        return Ok(Publish {
            package: package,
            version: body.version,
        });

        #[derive(Debug, Clone, Deserialize)]
        pub struct ImPublish {
            pub version: Version,
        }
    }
}

impl TryFromToml for RpRequiredPackage {
    fn try_from_string(_: &Path, id: &str, value: String) -> Result<Self> {
        let package = RpPackage::parse(id);

        let range =
            Range::parse(value.as_str()).map_err(|e| format!("bad version: {}: {}", e, value))?;

        Ok(RpRequiredPackage::new(package, range))
    }

    fn try_from_value(_: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let package = RpPackage::parse(id);
        let body: Package = value.try_into()?;
        let range = body.version.unwrap_or_else(Range::any);

        Ok(RpRequiredPackage::new(package, range))
    }
}

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
    version: Option<Range>,
}

/// Parse a single specification where the string key is a package.
///
/// The behavior for the value is determined by `TryFromToml`.
pub fn parse_spec<T: 'static>(base: &Path, id: &str, value: toml::Value) -> Result<T>
where
    T: TryFromToml,
{
    use self::toml::Value::*;

    match value {
        String(value) => T::try_from_string(base, id, value),
        value => T::try_from_value(base, id, value),
    }
}

/// Parse multiple speicifcations where the keys are packages.
pub fn parse_specs<T: 'static>(base: &Path, value: toml::Value) -> Result<Vec<T>>
where
    T: TryFromToml,
{
    let mut packages = Vec::new();
    let values = value.try_into::<HashMap<String, toml::Value>>()?;

    for (name, value) in values.into_iter() {
        packages.push(parse_spec(base, name.as_str(), value)?);
    }

    Ok(packages)
}

/// Parse optional specs.
pub fn opt_specs<T: 'static>(base: &Path, value: Option<toml::Value>) -> Result<Vec<T>>
where
    T: TryFromToml,
{
    value.map(|v| parse_specs(base, v)).unwrap_or(Ok(vec![]))
}

/// A quick bundle of configuration that can be applied, depending on what the project looks like.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Preset {
    Maven {},
}

impl Preset {}

/// Apply the given preset to a manifest.
fn apply_preset_to<L>(value: toml::Value, manifest: &mut Manifest<L>, base: &Path) -> Result<()>
where
    L: Lang,
{
    use self::Preset::*;
    use self::toml::Value::*;

    match value {
        String(name) => match name.as_str() {
            "maven" => maven_apply_to(manifest, base)?,
            name => return Err(format!("unsupported preset: {}", name).into()),
        },
        value => {
            let preset: Preset = value.try_into()?;

            match preset {
                Maven { .. } => maven_apply_to(manifest, base)?,
            }
        }
    }

    return Ok(());

    fn maven_apply_to<L>(manifest: &mut Manifest<L>, base: &Path) -> Result<()>
    where
        L: Lang,
    {
        // default path
        manifest
            .paths
            .push(base.join("src").join("main").join("reproto"));

        // output directory
        manifest.output = Some(
            base.join("target")
                .join("generated")
                .join("reproto")
                .join("java"),
        );

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Doc {
    /// Syntax theme to use.
    pub syntax_theme: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Publish {
    pub package: RpPackage,
    pub version: Version,
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

/// The first part when the manifest was read.
#[derive(Debug, Clone, Default)]
pub struct ManifestPreamble {
    pub language: Option<Language>,
    pub path: Option<PathBuf>,
    value: toml::value::Table,
}

impl ManifestPreamble {
    pub fn new(language: Option<Language>, path: Option<&Path>) -> ManifestPreamble {
        ManifestPreamble {
            language: language,
            path: path.map(|p| p.to_owned()),
            value: toml::value::Table::default(),
        }
    }
}

/// The realized project manifest.
///
/// * All paths are absolute.
#[derive(Debug, Clone)]
pub struct Manifest<L = NoLang>
where
    L: Lang,
{
    /// Path where manifest was loaded from.
    pub path: Option<PathBuf>,
    /// Packages to build.
    pub packages: Vec<RpRequiredPackage>,
    /// Files to build.
    pub files: Vec<ManifestFile>,
    /// Read files from stdin.
    ///
    /// This is not part of the manifest.
    pub stdin: bool,
    /// Packages to publish.
    pub publish: Vec<Publish>,
    /// Modules to enable.
    pub modules: Vec<L::Module>,
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
    /// Documentation settings.
    pub doc: Doc,
}

impl<L> Manifest<L>
where
    L: Lang,
{
    pub fn new(path: Option<&Path>) -> Manifest<L> {
        Manifest {
            path: path.map(|p| p.to_owned()),
            packages: Vec::default(),
            files: Vec::default(),
            stdin: false,
            publish: Vec::default(),
            modules: Vec::default(),
            paths: Vec::default(),
            output: Option::default(),
            package_prefix: Option::default(),
            id_converter: Option::default(),
            repository: Repository::default(),
            doc: Doc::default(),
        }
    }
}

/// Load and apply all repository-specific information.
pub fn load_repository(
    repository: &mut Repository,
    _base: &Path,
    value: &mut toml::value::Table,
) -> Result<()> {
    repository.no_repository = take_field(value, "no_repository")?;
    repository.index = take_field(value, "index")?;
    repository.objects = take_field(value, "objects")?;
    Ok(())
}

fn take_field<'de, T>(value: &mut toml::value::Table, name: &str) -> Result<T>
where
    T: Default + serde::Deserialize<'de>,
{
    if let Some(field) = value.remove(name) {
        field
            .try_into()
            .map_err(|e| format!("{}: {}", name, e).into())
    } else {
        Ok(T::default())
    }
}

fn check_empty(value: &toml::value::Table) -> Result<()> {
    let unexpected: Vec<String> = value.keys().map(Clone::clone).collect();

    if unexpected.len() > 0 {
        return Err(format!("unexpected entries: {}", unexpected.join(", ")).into());
    }

    Ok(())
}

fn take_section<F>(value: &mut toml::value::Table, name: &str, mut func: F) -> Result<()>
where
    F: FnMut(&mut toml::value::Table) -> Result<()>,
{
    let mut inner = take_field::<toml::value::Table>(value, "repository")?;
    func(&mut inner)?;
    check_empty(&inner).map_err(|e| format!("{}: {}", name, e.display()))?;
    Ok(())
}

/// Load and apply all options to the given file manifest to build a realized manifest.
///
/// `manifest` is the manifest that will be populated.
/// `base` is the base directory for which all paths specified in the manifest will be resolved.
pub fn load_common_manifest<L>(
    manifest: &mut Manifest<L>,
    base: &Path,
    value: &mut toml::value::Table,
) -> Result<()>
where
    L: Lang,
{
    manifest
        .packages
        .extend(opt_specs(base, take_field(value, "packages")?)?);
    manifest
        .files
        .extend(opt_specs(base, take_field(value, "files")?)?);
    manifest
        .publish
        .extend(opt_specs(base, take_field(value, "publish")?)?);

    manifest.paths.extend(
        take_field::<Vec<RelativePathBuf>>(value, "paths")?
            .iter()
            .map(|r| r.to_path(&base)),
    );

    if let Some(output) = take_field::<Option<RelativePathBuf>>(value, "output")? {
        manifest.output = Some(output.to_path(base));
    }

    for preset in take_field::<Vec<toml::Value>>(value, "presets")? {
        apply_preset_to(preset, manifest, &base)?;
    }

    if let Some(package_prefix) = take_field::<Option<RpPackage>>(value, "package_prefix")? {
        manifest.package_prefix = Some(package_prefix);
    }

    if let Some(id_converter) = take_field::<Option<String>>(value, "id_converter")? {
        manifest.id_converter = Some(id_converter);
    }

    take_section(value, "repository", |repository| {
        load_repository(&mut manifest.repository, base, repository)
    })?;

    if let Some(doc) = take_field::<Option<Doc>>(value, "doc")? {
        manifest.doc = doc;
    }

    Ok(())
}

/// Read and parse the manifest as TOML, extracting the language (if present) in the process.
pub fn read_manifest_preamble<'a, P: AsRef<Path>, R: Read>(
    path: P,
    mut reader: R,
) -> Result<ManifestPreamble> {
    let path = path.as_ref();

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mut value: toml::value::Table = toml::from_str(content.as_str())
        .map_err(|e| format!("{}: bad manifest: {}", path.display(), e))?;

    let language = take_field::<Option<Language>>(&mut value, "language")?;

    Ok(ManifestPreamble {
        language: language,
        path: Some(path.to_owned()),
        value: value,
    })
}

/// Read the given manifest.
///
/// Takes a path since it's used to convert declarations.
/// Returns `true` if the manifest is present, `false` otherwise.
pub fn read_manifest<L>(mut preamble: ManifestPreamble) -> Result<Manifest<L>>
where
    L: Lang,
{
    let mut manifest = Manifest::new(preamble.path.as_ref().map(AsRef::as_ref));

    // Only load components if we have a parent path.
    if let Some(parent) = preamble.path.as_ref().and_then(|p| p.parent()) {
        manifest.modules.extend(opt_specs(
            parent,
            take_field(&mut preamble.value, "modules")?,
        )?);

        load_common_manifest(&mut manifest, parent, &mut preamble.value)?;
    }

    check_empty(&preamble.value)?;
    Ok(manifest)
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
            let path = Path::new(".").join($name);

            let preamble = read_manifest_preamble(
                &path, Cursor::new(include_vec!($name)))
                .expect("to read preamble of manifest");

            read_manifest::<NoLang>(preamble).expect("to read manifest")
        }}
    }

    #[test]
    pub fn test_paths() {
        let manifest = include_manifest!("tests/paths.reproto");
        assert_eq!(1, manifest.paths.len());
    }

    #[test]
    pub fn test_presets_string() {
        let manifest = include_manifest!("tests/presets_string.reproto");
        assert_eq!(1, manifest.paths.len());
    }

    #[test]
    pub fn test_presets_section() {
        let manifest = include_manifest!("tests/presets_section.reproto");
        assert_eq!(1, manifest.paths.len());
    }

    #[test]
    pub fn test_packages_string() {
        let manifest = include_manifest!("tests/packages_string.reproto");
        assert_eq!(1, manifest.packages.len());
    }

    #[test]
    pub fn test_packages_table() {
        let manifest = include_manifest!("tests/packages_table.reproto");
        assert_eq!(1, manifest.packages.len());
    }

    #[test]
    pub fn test_packages_table2() {
        let manifest = include_manifest!("tests/packages_table2.reproto");
        assert_eq!(1, manifest.packages.len());
    }

    #[test]
    pub fn test_publish_string() {
        let manifest = include_manifest!("tests/publish_string.reproto");
        assert_eq!(1, manifest.publish.len());
    }

    #[test]
    pub fn test_publish_table() {
        let manifest = include_manifest!("tests/publish_table.reproto");
        assert_eq!(1, manifest.publish.len());
    }

    #[test]
    pub fn test_publish_table2() {
        let manifest = include_manifest!("tests/publish_table2.reproto");
        assert_eq!(1, manifest.publish.len());
    }

    #[test]
    pub fn test_files_string() {
        let manifest = include_manifest!("tests/files_string.reproto");
        assert_eq!(1, manifest.files.len());
    }

    #[test]
    pub fn test_files_table() {
        let manifest = include_manifest!("tests/files_table.reproto");
        assert_eq!(1, manifest.files.len());
    }

    #[test]
    pub fn test_files_table2() {
        let manifest = include_manifest!("tests/files_table2.reproto");
        assert_eq!(1, manifest.files.len());
    }

    #[test]
    pub fn test_empty() {
        let manifest = include_manifest!("tests/empty.reproto");

        assert_eq!(0, manifest.publish.len());
        assert_eq!(0, manifest.packages.len());
        assert_eq!(0, manifest.files.len());
    }

    #[test]
    pub fn test_repository() {
        let manifest = include_manifest!("tests/repository.reproto");

        assert_eq!(true, manifest.repository.no_repository);
        assert_eq!(
            Some("file:///index"),
            manifest.repository.index.as_ref().map(String::as_str)
        );
        assert_eq!(
            Some("file:///objects"),
            manifest.repository.objects.as_ref().map(String::as_str)
        );
    }
}
