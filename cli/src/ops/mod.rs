mod build;
mod doc;
mod config_env;
mod imports;
mod manifest;
mod publish;
mod repo;
mod update;
mod verify;
mod check;

use self::config_env::ConfigEnv;
use self::imports::*;
use backend::{CamelCase, FromNaming, Naming, SnakeCase};
use core::{Object, RpPackage, RpPackageFormat, RpVersionedPackage, Version};
use manifest::{Manifest, ManifestFile, Publish, read_manifest, self as m};
use relative_path::RelativePath;
use repository::*;
use semck;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;
use url;

pub const DEFAULT_INDEX: &'static str = "git+https://github.com/reproto/reproto-index";
pub const MANIFEST_NAME: &'static str = "reproto.toml";

fn parse_id_converter(input: &str) -> Result<Box<Naming>> {
    let mut parts = input.split(":");

    if let Some(first) = parts.next() {
        if let Some(second) = parts.next() {
            let naming: Box<FromNaming> = match first {
                "camel" => Box::new(CamelCase::new()),
                "snake" => Box::new(SnakeCase::new()),
                _ => {
                    return Err(
                        format!(
                            "Not a valid source: {}, must be one of: camel, snake",
                            first
                        ).into(),
                    )
                }
            };

            let naming = match second {
                "lower_camel" => naming.to_lower_camel(),
                "upper_camel" => naming.to_upper_camel(),
                "lower_snake" => naming.to_lower_snake(),
                "upper_snake" => naming.to_upper_snake(),
                _ => {
                    return Err(
                        format!(
                            "Not a valid target: {}, must be one of: lower_camel, upper_camel, \
                             lower_snake, upper_snake",
                            second
                        ).into(),
                    )
                }
            };

            return Ok(naming);
        }
    }

    return Err(
        format!("Invalid ID conversion `{}`, expected <from>:<to>", input).into(),
    );
}

pub fn base_args<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.arg(
        Arg::with_name("index")
            .long("index")
            .short("I")
            .takes_value(true)
            .help("URL for index to use when looking up packages."),
    );

    let out = out.arg(
        Arg::with_name("no-repository")
            .long("no-repository")
            .takes_value(false)
            .help("Completely disable repository operations"),
    );

    let out = out.arg(
        Arg::with_name("objects")
            .long("objects")
            .short("O")
            .takes_value(true)
            .help("URL for objects storage to use when looking up packages."),
    );

    let out = out.arg(
        Arg::with_name("path")
            .long("path")
            .short("p")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1)
            .help("Paths to look for definitions."),
    );

    let out = out.arg(
        Arg::with_name("manifest-path")
            .long("manifest-path")
            .takes_value(true)
            .help("Path to manifest to build"),
    );

    out
}

/// Setup base compiler options.
pub fn build_args<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = base_args(out);

    let out = out.arg(
        Arg::with_name("package")
            .long("package")
            .help("Packages to compile")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1),
    );

    let out = out.arg(
        Arg::with_name("module")
            .long("module")
            .short("m")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1)
            .help("Modules to load for a given backend"),
    );

    let out = out.arg(
        Arg::with_name("id-converter")
            .long("id-converter")
            .takes_value(true)
            .help("Conversion method to use when naming fields by default"),
    );

    let out = out.arg(
        Arg::with_name("package-prefix")
            .long("package-prefix")
            .takes_value(true)
            .help("Package prefix to use when generating classes"),
    );

    let out = out.arg(
        Arg::with_name("file")
            .long("file")
            .help("File to compile")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1),
    );

    let out = out.arg(
        Arg::with_name("out")
            .long("out")
            .short("o")
            .takes_value(true)
            .help("Output directory"),
    );

    out
}

pub fn setup_compiler_options(
    manifest: &Manifest,
    matches: &ArgMatches,
) -> Result<CompilerOptions> {
    // output path as specified in manifest.
    let manifest_out = manifest.output.as_ref().map(PathBuf::as_path);

    // final output path
    let out_path = matches
        .value_of("out")
        .map(Path::new)
        .or(manifest_out)
        .ok_or("--out <dir>, or `output` key in manifest is required")?;

    Ok(CompilerOptions { out_path: out_path.to_owned() })
}

pub fn setup_repository(manifest: &Manifest) -> Result<Repository> {
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

    let index_config = IndexConfig { repo_dir: repo_dir.clone() };

    let objects_config = ObjectsConfig {
        repo_dir: repo_dir,
        cache_dir: cache_dir,
        missing_cache_time: Some(Duration::new(60, 0)),
    };

    let index_url = index.unwrap_or_else(|| DEFAULT_INDEX.to_owned());

    let index = match url::Url::parse(index_url.as_ref()) {
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let path = RelativePath::new(index_url.as_str()).to_path(base);
            index_from_path(&path)?
        }
        Err(e) => return Err(e.into()),
        Ok(url) => index_from_url(index_config, &url)?,
    };

    let objects = {
        let objects_url = if let Some(ref objects) = objects {
            objects.as_ref()
        } else {
            index.objects_url()?
        };

        debug!("index: {}", index_url);
        debug!("objects: {}", objects_url);

        match url::Url::parse(objects_url) {
            // Relative to index index repository!
            Err(url::ParseError::RelativeUrlWithoutBase) => {
                index.objects_from_index(RelativePath::new(objects_url))?
            }
            Err(e) => return Err(e.into()),
            Ok(url) => objects_from_url(objects_config, &url)?,
        }
    };

    Ok(Repository::new(index, objects))
}

pub fn setup_path_resolver(manifest: &Manifest) -> Result<Option<Box<Resolver>>> {
    if manifest.paths.is_empty() {
        return Ok(None);
    }

    Ok(Some(Box::new(Paths::new(manifest.paths.clone()))))
}

pub fn setup_resolvers(manifest: &Manifest) -> Result<Box<Resolver>> {
    let mut resolvers: Vec<Box<Resolver>> = Vec::new();

    if let Some(resolver) = setup_path_resolver(manifest)? {
        resolvers.push(resolver);
    }

    resolvers.push(Box::new(setup_repository(manifest)?));

    Ok(Box::new(Resolvers::new(resolvers)))
}

pub fn setup_options(manifest: &Manifest) -> Result<Options> {
    let id_converter = if let Some(id_converter) = manifest.id_converter.as_ref() {
        Some(parse_id_converter(id_converter)?)
    } else {
        None
    };

    Ok(Options {
        id_converter: id_converter,
        modules: manifest.modules.clone(),
    })
}

pub fn setup_environment(manifest: &Manifest) -> Result<Environment> {
    let resolvers = setup_resolvers(manifest)?;
    let package_prefix = manifest.package_prefix.clone();
    Ok(Environment::new(package_prefix, resolvers))
}

/// Read the manifest based on the current environment.
pub fn setup_manifest<'a>(matches: &ArgMatches<'a>) -> Result<Manifest> {
    let manifest_path = matches
        .value_of("manifest-path")
        .map::<Result<&Path>, _>(|p| Ok(Path::new(p)))
        .unwrap_or_else(|| Ok(Path::new(MANIFEST_NAME)))?;

    let mut manifest = Manifest::new(manifest_path);

    if manifest_path.is_file() {
        debug!("reading manifest: {}", manifest_path.display());
        let reader = File::open(manifest_path.clone())?;
        read_manifest(&mut manifest, &manifest_path, reader)
            .map_err(|e| format!("{}: {}", manifest_path.display(), e))?;
    }

    manifest_from_matches(&mut manifest, matches)?;
    Ok(manifest)
}

pub fn setup_env(manifest: &Manifest) -> Result<Environment> {
    let mut env = setup_environment(manifest)?;

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

pub fn options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.subcommand(build_args(build::options()));
    let out = out.subcommand(build_args(verify::options()));
    let out = out.subcommand(build_args(doc::options()));
    let out = out.subcommand(base_args(check::options()));
    let out = out.subcommand(base_args(publish::options()));
    let out = out.subcommand(base_args(update::options()));
    let out = out.subcommand(base_args(repo::options()));
    let out = out.subcommand(base_args(manifest::options()));
    out
}

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

fn manifest_from_matches(manifest: &mut Manifest, matches: &ArgMatches) -> Result<()> {
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

    manifest.modules.extend(
        matches
            .values_of("module")
            .into_iter()
            .flat_map(|it| it)
            .map(ToOwned::to_owned),
    );

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

    repository_from_matches(&mut manifest.repository, matches)?;
    Ok(())
}

/// Argument match.
pub struct Match(Version, Box<Object>, RpPackage);

/// Formatting of candidate.
struct DisplayMatch<'a>(&'a (Option<Version>, Box<Object>));

impl<'a> fmt::Display for DisplayMatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = &self.0;

        if let Some(ref version) = inner.0 {
            write!(f, "{}@{}", inner.1, version)
        } else {
            write!(f, "{}@*", inner.1)
        }
    }
}

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
        let package = RpRequiredPackage::new(publish.package.clone(), None);
        let resolved = resolver.resolve(&package)?;

        if resolved.is_empty() {
            return Err(
                format!("no matching packages found for: {}", package).into(),
            );
        }

        // packages.push(RpRequiredPackage());
        for (_, object) in resolved {
            let version = version_override.unwrap_or(&publish.version).clone();
            results.push(Match(version, object, publish.package.clone()));
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
            warn!("matched: {}", DisplayMatch(&first));
            warn!("    and: {}", DisplayMatch(&next));

            while let Some(next) = it.next() {
                warn!("    and: {}", DisplayMatch(&next));
            }

            return Err("more than one matching package found".into());
        }

        let (version, object) = first;

        let version = version.ok_or_else(
            || format!("{}: package without a version", object),
        )?;

        let version = version_override.map(Clone::clone).unwrap_or(version);
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

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    match name {
        "build" => return build::entry(matches),
        "verify" => return verify::entry(matches),
        "check" => return check::entry(matches),
        "doc" => return doc::entry(matches),
        "update" => return update::entry(matches),
        "publish" => return publish::entry(matches),
        "repo" => return repo::entry(matches),
        "manifest" => return manifest::entry(matches),
        _ => {}
    }

    Err(format!("No such command: {}", name).into())
}
