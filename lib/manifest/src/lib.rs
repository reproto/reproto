//! Functions and data-structures for loading a project manifest.
//!
//! Project manifests can be loaded as a convenient method for setting up language or
//! project-specific configuration for reproto.

use naming::Naming;
use relative_path::{RelativePath, RelativePathBuf};
use reproto_core::errors::Result;
use reproto_core::{
    CoreFlavor, Range, Resolved, ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage,
    RpVersionedPackage, Version,
};
use serde::Deserialize;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! lang_base {
    ($module:ty, $compile:ident) => {
        fn copy(&self) -> Box<dyn Lang> {
            Box::new(*self)
        }

        /// Module specs.
        fn module_specs(
            &self,
            path: &Path,
            input: Option<toml::Value>,
        ) -> Result<Option<Vec<Box<dyn Any>>>> {
            $crate::parse_section_any::<$module>(path, input)
        }

        fn string_spec(&self, path: &Path, input: &str) -> Result<Box<dyn Any>> {
            $crate::parse_string_any::<$module>(path, input)
        }

        fn compile(
            &self,
            handle: &dyn reproto_core::Handle,
            env: trans::Session<reproto_core::CoreFlavor>,
            manifest: $crate::Manifest,
        ) -> Result<()> {
            $compile(handle, env, manifest)
        }
    };
}

/// The trait that describes the specific implementation of a given language.
///
/// TODO: move language-specific integrations into own crate.
/// Options would have to be transferred to a local type.
pub trait Lang: fmt::Debug {
    /// Copy self.
    ///
    /// Implemented through `lang_base!` macro.
    fn copy(&self) -> Box<dyn Lang>;

    /// Parse a complex set of module configurations.
    ///
    /// Implemented through `lang_base!` macro.
    fn module_specs(
        &self,
        path: &Path,
        input: Option<toml::Value>,
    ) -> Result<Option<Vec<Box<dyn Any>>>>;

    /// Parse a module configuration consisting of _only_ a string.
    ///
    /// Implemented through `lang_base!` macro.
    fn string_spec(&self, path: &Path, input: &str) -> Result<Box<dyn Any>>;

    /// Language-specific compile hook.
    ///
    /// Implemented through `lang_base!` macro.
    fn compile(
        &self,
        handle: &dyn reproto_core::Handle,
        env: trans::Session<CoreFlavor>,
        manifest: Manifest,
    ) -> Result<()>;

    /// Comment the given string.
    fn comment(&self, _input: &str) -> Option<String> {
        None
    }

    /// Get a list of keywords to transliterate.
    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![]
    }

    /// Indicates if the language requires keyword-escaping in the packages.
    fn safe_packages(&self) -> bool {
        false
    }

    /// Helper to convert into session.
    fn into_session<'a>(
        &self,
        package_prefix: Option<RpPackage>,
        reporter: &'a mut dyn reproto_core::Reporter,
        resolver: &'a mut dyn reproto_core::Resolver,
    ) -> Result<trans::Session<'a, CoreFlavor>> {
        let keywords = self
            .keywords()
            .into_iter()
            .map(|(f, t)| (f.to_string(), t.to_string()))
            .collect();

        let session = trans::Session::new(package_prefix.clone(), reporter, resolver)?
            .with_keywords(keywords)
            .with_safe_packages(self.safe_packages());

        let session = if let Some(package_naming) = self.package_naming() {
            session.with_package_naming(package_naming)
        } else {
            session
        };

        let session = if let Some(field_ident_naming) = self.field_ident_naming() {
            session.with_field_ident_naming(field_ident_naming)
        } else {
            session
        };

        let session = if let Some(endpoint_ident_naming) = self.endpoint_ident_naming() {
            session.with_endpoint_ident_naming(endpoint_ident_naming)
        } else {
            session
        };

        Ok(session)
    }

    /// Rename packages according to the given naming convention.
    fn package_naming(&self) -> Option<Box<dyn Naming>> {
        None
    }

    /// Rename fields according to the given naming convention.
    fn field_ident_naming(&self) -> Option<Box<dyn Naming>> {
        None
    }

    /// Rename endpoint identifiers according to the given naming convention.
    fn endpoint_ident_naming(&self) -> Option<Box<dyn Naming>> {
        None
    }
}

/// Fallback language support in case no language is specified.
#[derive(Clone, Copy, Debug)]
pub struct NoLang;

pub enum NoModule {}

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
    lang_base!(NoModule, no_compile);
}

fn no_compile(
    _handle: &dyn reproto_core::Handle,
    _env: trans::Session<CoreFlavor>,
    _manifest: Manifest,
) -> Result<()> {
    Ok(())
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

impl TryFromToml for File {
    fn try_from_string(base: &Path, id: &str, value: String) -> Result<Self> {
        let package = RpPackage::parse(id);
        let path = RelativePath::new(value.as_str()).to_path(base);

        Ok(File {
            path: path,
            package: Some(package),
            version: None,
        })
    }

    fn try_from_value(base: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let package = RpPackage::parse(id);
        let body: ImFile = value.try_into()?;

        return Ok(File {
            path: body.path.to_path(base),
            package: Some(package),
            version: body.version,
        });

        #[derive(Debug, Clone, Deserialize)]
        pub struct ImFile {
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
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Csharp,
    Dart,
    Go,
    Java,
    Js,
    Json,
    OpenApi,
    Python,
    Reproto,
    Rust,
    Swift,
}

impl Language {
    pub fn parse(input: &str) -> Option<Language> {
        use self::Language::*;

        let language = match input {
            "csharp" => Csharp,
            "dart" => Dart,
            "go" => Go,
            "java" => Java,
            "js" => Js,
            "json" => Json,
            "openapi" => OpenApi,
            "python" => Python,
            "reproto" => Reproto,
            "rust" => Rust,
            "swift" => Swift,
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
pub fn parse_spec<T>(base: &Path, id: &str, value: toml::Value) -> Result<T>
where
    T: TryFromToml,
{
    match value {
        toml::Value::String(value) => T::try_from_string(base, id, value),
        value => T::try_from_value(base, id, value),
    }
}

/// Parse multiple speicifcations where the keys are packages.
pub fn parse_specs<T>(base: &Path, value: toml::Value) -> Result<Option<Vec<T>>>
where
    T: TryFromToml,
{
    if let Some(values) = value.try_into::<Option<HashMap<String, toml::Value>>>()? {
        let mut packages = Vec::new();

        for (name, value) in values.into_iter() {
            packages.push(parse_spec(base, name.as_str(), value)?);
        }

        return Ok(Some(packages));
    }

    Ok(None)
}

/// Parse optional specs.
fn parse_section<T>(base: &Path, value: Option<toml::Value>) -> Result<Option<Vec<T>>>
where
    T: TryFromToml,
{
    if let Some(value) = value {
        return parse_specs(base, value);
    }

    Ok(None)
}

/// Parsing modules into Any.
pub fn parse_section_any<'a, T>(
    base: &Path,
    value: Option<toml::Value>,
) -> Result<Option<Vec<Box<dyn Any + 'a>>>>
where
    T: 'a + TryFromToml,
{
    if let Some(values) = parse_section::<T>(base, value)? {
        Ok(Some(
            values
                .into_iter()
                .map(|b| Box::new(b) as Box<dyn Any>)
                .collect(),
        ))
    } else {
        Ok(None)
    }
}

/// Parse the given string as a module.
pub fn parse_string_any<'a, T>(base: &Path, name: &str) -> Result<Box<dyn Any + 'a>>
where
    T: 'a + TryFromToml,
{
    let value = toml::Value::Table(toml::value::Table::default());
    Ok(Box::new(parse_spec::<T>(base, name, value)?) as Box<dyn Any>)
}

/// Attempt to perform a checked conversion of the given vector of modules.
pub fn checked_modules<M: Any>(modules: Option<Vec<Box<dyn Any>>>) -> Result<Vec<M>> {
    let mut out = Vec::new();

    if let Some(modules) = modules {
        for m in modules {
            out.push(
                *m.downcast::<M>()
                    .map_err(|m| format!("Failed to downcast module: {:?}", m))?,
            );
        }
    }

    Ok(out)
}

/// A quick bundle of configuration that can be applied, depending on what the project looks like.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Preset {
    Go {},
    Maven {},
    Swift {},
    Rust {},
}

impl TryFromToml for Preset {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        let preset = match id {
            "go" => Preset::Go {},
            "maven" => Preset::Maven {},
            "swift" => Preset::Swift {},
            "rust" => Preset::Rust {},
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(preset)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let preset = match id {
            "go" => Preset::Go {},
            "maven" => Preset::Maven {},
            "swift" => Preset::Swift {},
            "rust" => Preset::Rust {},
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(preset)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct Doc {
    /// Syntax theme to use.
    pub syntax_theme: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Publish {
    pub package: RpPackage,
    pub version: Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub path: PathBuf,
    pub package: Option<RpPackage>,
    pub version: Option<Version>,
}

impl File {
    pub fn from_path(path: &Path) -> File {
        File {
            path: path.to_owned(),
            package: None,
            version: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Repository {
    /// Skip using local repository.
    pub no_repository: bool,
    /// URL to use for index.
    pub index: Option<String>,
    /// URL to use to objects storage.
    pub objects: Option<String>,
}

#[derive(Debug)]
pub struct Source {
    pub package: RpVersionedPackage,
    pub source: reproto_core::Source,
}

/// The realized project manifest.
///
/// * All paths are absolute.
#[derive(Debug, Default)]
pub struct Manifest {
    /// Language manifest is being compiled for.
    pub lang: Option<Box<dyn Lang>>,
    /// Path where manifest was loaded from.
    pub path: Option<PathBuf>,
    /// Packages to build.
    pub packages: Option<Vec<RpRequiredPackage>>,
    /// Files to build.
    pub files: Option<Vec<File>>,
    /// Read files from stdin.
    ///
    /// This is not part of the manifest.
    pub stdin: bool,
    /// Packages to publish.
    pub publish: Option<Vec<Publish>>,
    /// Modules to enable.
    pub modules: Option<Vec<Box<dyn Any>>>,
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

impl Manifest {
    /// Load from YAML.
    pub fn from_yaml<R, C>(&mut self, mut reader: R, convert_language: C) -> Result<()>
    where
        R: Read,
        C: Fn(Language) -> Box<dyn Lang>,
    {
        let mut value = {
            let mut content = String::new();
            reader.read_to_string(&mut content)?;
            toml::from_str(content.as_str())?
        };

        if let Some(lang) = take_field::<Option<Language>>(&mut value, "language")? {
            // Already set, do not override.
            if self.lang.is_none() {
                self.lang = Some(convert_language(lang));
            }
        }

        let modules = take_field(&mut value, "modules")?;

        // Only load components if we have a parent path.
        if let Some(path) = self.path.clone() {
            let parent = path
                .parent()
                .ok_or_else(|| format!("path does not have a parent: {}", path.display()))?;

            if let Some(lang) = self.lang.as_ref() {
                self.modules = lang.module_specs(parent, modules)?;
            }

            load_common_manifest(self, parent, &mut value)?;
        }

        check_empty(&value)?;
        Ok(())
    }

    /// Access language to build for.
    pub fn lang(&self) -> Option<Box<dyn Lang>> {
        self.lang.as_ref().map(|l| l.copy())
    }

    /// Access language to build for, or fall back to `NoLang` which is a no-operation language.
    pub fn lang_or_nolang(&self) -> Box<dyn Lang> {
        self.lang().unwrap_or_else(|| Box::new(NoLang))
    }

    /// Check if manifest has nothing to build.
    pub fn is_build_empty(&self) -> bool {
        if !self.files.as_ref().map(Vec::is_empty).unwrap_or(true) {
            return false;
        }

        if !self.packages.as_ref().map(Vec::is_empty).unwrap_or(true) {
            return false;
        }

        self.path.is_none()
    }

    /// Resolve all sources for this manifest.
    pub fn resolve(&self, resolver: &mut dyn Resolver) -> Result<Vec<Source>> {
        let mut sources = Vec::new();

        // if there are no packages, load from path resolver.
        if self.packages.is_none() {
            // only build unique packages, some resolvers will resolve the same version.
            let mut seen = HashSet::new();

            for ResolvedByPrefix { package, source } in resolver.resolve_packages()? {
                if !seen.insert(package.clone()) {
                    continue;
                }

                log::trace!("resolved package `{}` to build", package);
                sources.push(Source { package, source });
            }
        }

        for file in self.files.as_ref().iter().flat_map(|f| f.iter()) {
            let package = file.package.clone().unwrap_or_else(RpPackage::empty);
            let package = RpVersionedPackage::new(package, file.version.clone());
            let source = reproto_core::Source::from_path(&file.path);
            sources.push(Source { package, source });
        }

        for required in self.packages.iter().flat_map(|p| p.iter()).cloned() {
            // find matching object from the resolver.
            let Resolved { version, source } = match resolver.resolve(&required)? {
                Some(resolved) => resolved,
                None => {
                    return Err(format!("required package `{}` not found", required).into());
                }
            };

            let package = RpVersionedPackage::new(required.package.clone(), version);
            sources.push(Source { package, source });
        }

        Ok(sources)
    }
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
pub fn load_common_manifest(
    manifest: &mut Manifest,
    base: &Path,
    value: &mut toml::value::Table,
) -> Result<()> {
    manifest.packages = parse_section(base, take_field(value, "packages")?)?;
    manifest.files = parse_section(base, take_field(value, "files")?)?;
    manifest.publish = parse_section(base, take_field(value, "publish")?)?;

    manifest.paths.extend(
        take_field::<Vec<RelativePathBuf>>(value, "paths")?
            .iter()
            .map(|r| r.to_path(&base)),
    );

    if let Some(output) = take_field::<Option<RelativePathBuf>>(value, "output")? {
        manifest.output = Some(output.to_path(base));
    }

    if let Some(presets) = parse_section(base, take_field(value, "presets")?)? {
        for preset in presets {
            apply_preset_to(preset, manifest, &base)?;
        }
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

    return Ok(());

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

    /// Apply the given preset to a manifest.
    fn apply_preset_to(preset: Preset, manifest: &mut Manifest, base: &Path) -> Result<()> {
        use self::Preset::*;

        match preset {
            Go { .. } => go_apply_to(manifest, base)?,
            Maven { .. } => maven_apply_to(manifest, base)?,
            Swift { .. } => swift_apply_to(manifest, base)?,
            Rust { .. } => rust_apply_to(manifest, base)?,
        }

        return Ok(());

        fn maven_apply_to(manifest: &mut Manifest, base: &Path) -> Result<()> {
            // default path
            manifest
                .paths
                .push(base.join("src").join("main").join("reproto"));

            // output directory
            manifest.output = Some(
                base.join("target")
                    .join("generated-sources")
                    .join("reproto")
                    .join("java"),
            );

            Ok(())
        }

        fn swift_apply_to(manifest: &mut Manifest, base: &Path) -> Result<()> {
            // default path
            manifest.paths.push(base.join("reproto"));

            // output directory
            manifest.output = Some(base.join("Sources").join("Modules"));

            Ok(())
        }

        fn go_apply_to(manifest: &mut Manifest, base: &Path) -> Result<()> {
            // default path
            manifest.paths.push(base.join("reproto"));

            // output directory
            manifest.output = Some(base.join("models"));

            Ok(())
        }

        fn rust_apply_to(manifest: &mut Manifest, base: &Path) -> Result<()> {
            // output directory
            manifest.output = Some(base.join("src"));
            // package prefix
            manifest.package_prefix = Some(RpPackage::new(vec!["gen".to_string()]));

            Ok(())
        }
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
        }};
    }

    macro_rules! include_manifest {
        ($name:expr) => {{
            let mut manifest = Manifest::default();
            manifest.path = Some(Path::new(".").join($name));
            manifest
                .from_yaml(Cursor::new(include_vec!($name)), |_| Box::new(NoLang))
                .expect("failed to read manifest");
            manifest
        }};
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
        assert_eq!(Some(1), manifest.packages.map(|p| p.len()));
    }

    #[test]
    pub fn test_packages_table() {
        let manifest = include_manifest!("tests/packages_table.reproto");
        assert_eq!(Some(1), manifest.packages.map(|p| p.len()));
    }

    #[test]
    pub fn test_packages_table2() {
        let manifest = include_manifest!("tests/packages_table2.reproto");
        assert_eq!(Some(1), manifest.packages.map(|p| p.len()));
    }

    #[test]
    pub fn test_publish_string() {
        let manifest = include_manifest!("tests/publish_string.reproto");
        assert_eq!(Some(1), manifest.publish.map(|p| p.len()));
    }

    #[test]
    pub fn test_publish_table() {
        let manifest = include_manifest!("tests/publish_table.reproto");
        assert_eq!(Some(1), manifest.publish.map(|p| p.len()));
    }

    #[test]
    pub fn test_publish_table2() {
        let manifest = include_manifest!("tests/publish_table2.reproto");
        assert_eq!(Some(1), manifest.publish.map(|p| p.len()));
    }

    #[test]
    pub fn test_files_string() {
        let manifest = include_manifest!("tests/files_string.reproto");
        assert_eq!(Some(1), manifest.files.map(|p| p.len()));
    }

    #[test]
    pub fn test_files_table() {
        let manifest = include_manifest!("tests/files_table.reproto");
        assert_eq!(Some(1), manifest.files.map(|p| p.len()));
    }

    #[test]
    pub fn test_files_table2() {
        let manifest = include_manifest!("tests/files_table2.reproto");
        assert_eq!(Some(1), manifest.files.map(|p| p.len()));
    }

    #[test]
    pub fn test_empty() {
        let manifest = include_manifest!("tests/empty.reproto");

        assert_eq!(None, manifest.publish.map(|p| p.len()));
        assert_eq!(None, manifest.packages.map(|p| p.len()));
        assert_eq!(None, manifest.files.map(|f| f.len()));
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
