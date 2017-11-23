use backend::{self, Environment};
use clap::ArgMatches;
use config_env::ConfigEnv;
use core::{Context, Object, RpPackage, RpPackageFormat, RpRequiredPackage, RpVersionedPackage,
           Version};
use errors::*;
use manifest::{Lang, Manifest, ManifestFile, ManifestPreamble, Publish, TryFromToml,
               read_manifest, read_manifest_preamble, self as m};
use relative_path::RelativePath;
use repository::{Index, IndexConfig, NoIndex, NoObjects, Objects, ObjectsConfig, Paths,
                 Repository, Resolved, ResolvedByPrefix, Resolver, Resolvers, index_from_path,
                 index_from_url, objects_from_path, objects_from_url};
use semck;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;
use toml;
use url;

pub const DEFAULT_INDEX: &'static str = "git+https://github.com/reproto/reproto-index";
pub const MANIFEST_NAME: &'static str = "reproto.toml";

fn load_index(base: &Path, index_url: &str, config: IndexConfig) -> Result<Box<Index>> {
    let index_path = Path::new(index_url);

    if index_path.is_dir() {
        let index_path = index_path.canonicalize().map_err(|e| {
            format!("index: bad path: {}: {}", e, index_path.display())
        })?;

        return index_from_path(&index_path)
            .map(|i| Box::new(i) as Box<Index>)
            .map_err(Into::into);
    }

    match url::Url::parse(index_url) {
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let path = RelativePath::new(index_url).to_path(base);

            index_from_path(&path)
                .map(|i| Box::new(i) as Box<Index>)
                .map_err(Into::into)
        }
        Err(e) => return Err(e.into()),
        Ok(url) => index_from_url(config, &url).map_err(Into::into),
    }
}

fn load_objects(
    index: &Index,
    index_url: &str,
    objects: Option<String>,
    config: ObjectsConfig,
) -> Result<Box<Objects>> {
    let objects_url = if let Some(ref objects) = objects {
        objects.as_ref()
    } else {
        index.objects_url()?
    };

    debug!("index: {}", index_url);
    debug!("objects: {}", objects_url);

    let objects_path = Path::new(objects_url);

    if objects_path.is_dir() {
        let objects_path = objects_path.canonicalize().map_err(|e| {
            format!("objects: bad path: {}: {}", e, objects_path.display())
        })?;

        return objects_from_path(objects_path)
            .map(|o| Box::new(o) as Box<Objects>)
            .map_err(Into::into);
    }

    match url::Url::parse(objects_url) {
        // Relative to index index repository!
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            index
                .objects_from_index(RelativePath::new(objects_url))
                .map_err(Into::into)
        }
        Err(e) => return Err(e.into()),
        Ok(url) => objects_from_url(config, &url).map_err(Into::into),
    }
}

pub fn setup_repository<L>(manifest: &Manifest<L>) -> Result<Repository>
where
    L: Lang,
{
    let repository = &manifest.repository;

    if repository.no_repository {
        return Ok(Repository::new(Box::new(NoIndex), Box::new(NoObjects)));
    }

    let base = manifest.path.parent().ok_or_else(
        || "no parent path to manifest",
    )?;

    let mut repo_dir = None;
    let mut cache_dir = None;
    let mut index = repository.index.clone();
    let mut objects = repository.objects.clone();

    if let Some(config_env) = ConfigEnv::new()? {
        repo_dir = Some(config_env.repo_dir);
        cache_dir = Some(config_env.cache_dir);
        index = index.or(config_env.index.clone());
        objects = objects.or(config_env.objects.clone());
    }

    let repo_dir = repo_dir.ok_or_else(|| "repo_dir: must be specified")?;

    let index_url = index.unwrap_or_else(|| DEFAULT_INDEX.to_owned());
    let index_config = IndexConfig { repo_dir: repo_dir.clone() };

    let index = load_index(base, index_url.as_str(), index_config)?;

    let objects_config = ObjectsConfig {
        repo_dir: repo_dir,
        cache_dir: cache_dir,
        missing_cache_time: Some(Duration::new(60, 0)),
    };

    let objects = load_objects(index.as_ref(), index_url.as_str(), objects, objects_config)?;
    Ok(Repository::new(index, objects))
}

pub fn setup_path_resolver<L>(manifest: &Manifest<L>) -> Result<Option<Box<Resolver>>>
where
    L: Lang,
{
    if manifest.paths.is_empty() {
        return Ok(None);
    }

    let mut published = HashMap::new();

    for publish in &manifest.publish {
        published.insert(publish.package.clone(), publish.version.clone());
    }

    Ok(Some(
        Box::new(Paths::new(manifest.paths.clone(), published)),
    ))
}

pub fn setup_resolvers<L>(manifest: &Manifest<L>) -> Result<Box<Resolver>>
where
    L: Lang,
{
    let mut resolvers: Vec<Box<Resolver>> = Vec::new();

    resolvers.push(Box::new(setup_repository(manifest)?));

    if let Some(resolver) = setup_path_resolver(manifest)? {
        resolvers.push(resolver);
    }

    Ok(Box::new(Resolvers::new(resolvers)))
}

/// Read the first part of the manifest, to determine the language used.
pub fn manifest_preamble<'a>(matches: &ArgMatches<'a>) -> Result<ManifestPreamble> {
    let manifest_path = matches
        .value_of("manifest-path")
        .map::<Result<&Path>, _>(|p| Ok(Path::new(p)))
        .unwrap_or_else(|| Ok(Path::new(MANIFEST_NAME)))?;

    if !manifest_path.is_file() {
        return Ok(ManifestPreamble::new(manifest_path));
    }

    debug!("reading manifest: {}", manifest_path.display());
    let reader = File::open(manifest_path.clone())?;

    read_manifest_preamble(&manifest_path, reader).map_err(
        |e| {
            format!("{}: {}", manifest_path.display(), e).into()
        },
    )
}

/// Read the manifest based on the current environment.
pub fn manifest<'a, L>(matches: &ArgMatches<'a>, preamble: ManifestPreamble) -> Result<Manifest<L>>
where
    L: Lang,
{
    let path = preamble.path.clone();

    let mut manifest = read_manifest(preamble).map_err(|e| {
        format!("{}: {}", path.display(), e)
    })?;

    manifest_from_matches(&mut manifest, matches)?;
    Ok(manifest)
}

/// Setup environment.
pub fn setup_environment<L>(ctx: Rc<Context>, manifest: &Manifest<L>) -> Result<Environment>
where
    L: Lang,
{
    let resolvers = setup_resolvers(manifest)?;
    let package_prefix = manifest.package_prefix.clone();

    let mut env = Environment::new(ctx, package_prefix, resolvers);

    let mut errors = Vec::new();

    // TODO: use version and package from the provided file.
    for file in &manifest.files {
        let package = file.package.as_ref().map(|p| {
            RpVersionedPackage::new(p.clone(), file.version.clone())
        });

        if let Err(e) = env.import_file(&file.path, package) {
            errors.push(e.into());
        }
    }

    for package in manifest.packages.iter().cloned() {
        match env.import(&package) {
            Err(e) => errors.push(e.into()),
            Ok(None) => errors.push(format!("no matching package: {}", package).into()),
            _ => {}
        }
    }

    if let Err(e) = env.verify() {
        errors.push(e.into());
    }

    if !errors.is_empty() {
        return Err(ErrorKind::Errors(errors).into());
    }

    Ok(env)
}

/// Populate the repository structure from CLI arguments.
///
/// CLI arguments take precedence.
fn repository_from_matches(repository: &mut m::Repository, matches: &ArgMatches) -> Result<()> {
    repository.no_repository = repository.no_repository || matches.is_present("no-repository");

    if let Some(objects) = matches.value_of("objects").map(ToOwned::to_owned) {
        repository.objects = Some(objects);
    }

    if let Some(index) = matches.value_of("index").map(ToOwned::to_owned) {
        repository.index = Some(index);
    }

    Ok(())
}

fn manifest_from_matches<L>(manifest: &mut Manifest<L>, matches: &ArgMatches) -> Result<()>
where
    L: Lang,
{
    manifest.paths.extend(
        matches
            .values_of("path")
            .into_iter()
            .flat_map(|it| it)
            .map(Path::new)
            .map(ToOwned::to_owned),
    );

    manifest.files.extend(
        matches
            .values_of("file")
            .into_iter()
            .flat_map(|it| it)
            .map(Path::new)
            .map(ManifestFile::from_path),
    );

    for module in matches.values_of("module").into_iter().flat_map(|it| it) {
        let module = L::Module::try_from_value(
            &manifest.path,
            module,
            toml::Value::Table(toml::value::Table::default()),
        )?;

        manifest.modules.push(module);
    }

    for package in matches.values_of("package").into_iter().flat_map(|it| it) {
        let parsed = RpRequiredPackage::parse(package);

        let parsed = parsed.chain_err(|| {
            format!("failed to parse --package argument: {}", package)
        })?;

        manifest.packages.push(parsed);
    }

    if let Some(package_prefix) = matches.value_of("package-prefix").map(RpPackage::parse) {
        manifest.package_prefix = Some(package_prefix);
    }

    if let Some(id_converter) = matches.value_of("id-converter") {
        manifest.id_converter = Some(id_converter.to_string());
    }

    // override output path
    if let Some(out) = matches.value_of("out").map(Path::new) {
        manifest.output = Some(out.to_owned());
    }

    repository_from_matches(&mut manifest.repository, matches)?;
    Ok(())
}

/// Argument match.
pub struct Match(pub Version, pub Box<Object>, pub RpPackage);

/// Setup matches from a publish manifest.
pub fn setup_publish_matches<'a, I>(
    resolver: &mut Resolver,
    version_override: Option<&Version>,
    publish: I,
) -> Result<Vec<Match>>
where
    I: IntoIterator<Item = &'a Publish>,
{
    let mut results = Vec::new();

    for publish in publish.into_iter() {
        let resolved = resolver.resolve_by_prefix(&publish.package)?;

        if resolved.is_empty() {
            return Err(
                format!("no matching packages found for: {}", publish.package).into(),
            );
        }

        for ResolvedByPrefix { package, object } in resolved {
            let version = version_override.unwrap_or(&publish.version).clone();
            results.push(Match(version, object, package.clone()));
        }
    }

    Ok(results)
}

pub fn setup_matches<'a, I>(
    resolver: &mut Resolver,
    version_override: Option<&Version>,
    packages: I,
) -> Result<Vec<Match>>
where
    I: IntoIterator<Item = &'a RpRequiredPackage>,
{
    let mut results = Vec::new();

    for package in packages.into_iter() {
        let resolved = resolver.resolve(package)?;

        if resolved.is_empty() {
            return Err(
                format!("no matching packages found for: {}", package).into(),
            );
        }

        let mut it = resolved.into_iter();

        let first = it.next().ok_or_else(|| format!("no packages to publish"))?;

        if let Some(next) = it.next() {
            warn!("matched: {}", first);
            warn!("    and: {}", next);

            while let Some(next) = it.next() {
                warn!("    and: {}", next);
            }

            return Err("more than one matching package found".into());
        }

        let Resolved { version, object } = first;
        let version = version_override.cloned().or(version);

        let version = version.ok_or_else(|| {
            ErrorKind::NoVersionToPublish(package.package.clone())
        })?;

        results.push(Match(version, object, package.package.clone()));
    }

    Ok(results)
}

pub fn semck_check(
    errors: &mut Vec<Error>,
    repository: &mut Repository,
    env: &mut Environment,
    m: &Match,
) -> Result<()> {
    let Match(ref version, ref object, ref package) = *m;

    // perform semck verification
    if let Some(d) = repository
        .all(package)?
        .into_iter()
        .filter(|d| d.version <= *version && !d.version.is_prerelease())
        .last()
    {
        debug!("Checking semantics of {} -> {}", d.version, version);

        let previous = repository.get_object(&d)?.ok_or_else(|| {
            format!("No object found for deployment: {:?}", d)
        })?;

        let name = RpPackageFormat(package, Some(&d.version)).to_string();
        let previous = previous.with_name(name);

        let package_from = RpVersionedPackage::new(package.clone(), Some(d.version.clone()));
        let file_from = env.load_object(previous, &package_from)?;

        let package_to = RpVersionedPackage::new(package.clone(), Some(version.clone()));
        let file_to = env.load_object(object.clone_object(), &package_to)?;

        let violations = semck::check((&d.version, &file_from), (&version, &file_to))?;

        if !violations.is_empty() {
            for (i, v) in violations.into_iter().enumerate() {
                errors.push(ErrorKind::SemckViolation(i, v).into());
            }
        }
    }

    Ok(())
}

/// High-level helper function to call the given clojure with all the necessary compile options.
pub fn manifest_compile<'a, L, F>(
    ctx: Rc<Context>,
    matches: &'a ArgMatches,
    preamble: ManifestPreamble,
    compile: F,
) -> Result<()>
where
    L: Lang,
    F: FnOnce(Rc<Context>, Environment, &'a ArgMatches, Manifest<L>) -> backend::errors::Result<()>,
{
    let manifest = manifest::<L>(matches, preamble)?;
    let env = setup_environment(ctx.clone(), &manifest)?;

    compile(ctx.clone(), env, matches, manifest)?;
    Ok(())
}

/// High-level helper function to call the given clojure with all the necessary compile options.
pub fn manifest_use<'a, L, F>(
    ctx: Rc<Context>,
    matches: &'a ArgMatches,
    preamble: ManifestPreamble,
    use_f: F,
) -> Result<()>
where
    L: Lang,
    F: FnOnce(Rc<Context>, &'a ArgMatches, Manifest<L>) -> Result<()>,
{
    let manifest = manifest::<L>(matches, preamble)?;
    use_f(ctx.clone(), matches, manifest)?;
    Ok(())
}
