mod compile;
mod doc;
mod config_env;
mod imports;
mod manifest;
mod publish;
mod repo;
mod update;
mod verify;

use self::config_env::ConfigEnv;
use self::imports::*;
use backend::{CamelCase, FromNaming, Naming, SnakeCase};
use manifest::{Manifest, read_manifest};
use repository::*;
use std::env;
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
                _ => return Err(format!("Not a valid source: {}", first).into()),
            };

            let naming = match second {
                "lower_camel" => naming.to_lower_camel(),
                "upper_camel" => naming.to_upper_camel(),
                "lower_snake" => naming.to_lower_snake(),
                "upper_snake" => naming.to_upper_snake(),
                _ => return Err(format!("Not a valid target: {}", second).into()),
            };

            return Ok(naming);
        }
    }

    return Err(
        format!("Invalid --id-conversion argument: {}", input).into(),
    );
}

pub fn path_base<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
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
pub fn compiler_base<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = path_base(out);

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

pub fn setup_repository(matches: &ArgMatches) -> Result<Repository> {
    if matches.is_present("no-repository") {
        return Ok(Repository::new(Box::new(NoIndex), Box::new(NoObjects)));
    }

    let mut repo_dir = None;
    let mut cache_dir = None;
    let mut index = matches.value_of("index").map(ToOwned::to_owned);
    let mut objects = matches.value_of("objects").map(ToOwned::to_owned);

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

    let index_url = url::Url::parse(index_url.as_ref())?;
    let index = index_from_url(index_config, &index_url)?;

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
                let relative_path = Path::new(objects_url);
                index.objects_from_index(&relative_path)?
            }
            Err(e) => return Err(e.into()),
            Ok(url) => objects_from_url(objects_config, &url)?,
        }
    };

    Ok(Repository::new(index, objects))
}

pub fn setup_path_resolver(
    manifest: &Manifest,
    matches: &ArgMatches,
) -> Result<Option<Box<Resolver>>> {
    let mut paths: Vec<PathBuf> = matches
        .values_of("path")
        .into_iter()
        .flat_map(|it| it)
        .map(Path::new)
        .map(ToOwned::to_owned)
        .collect();

    paths.extend(manifest.paths.iter().cloned());

    if paths.is_empty() {
        return Ok(None);
    }

    Ok(Some(Box::new(Paths::new(paths))))
}

pub fn setup_resolvers(manifest: &Manifest, matches: &ArgMatches) -> Result<Box<Resolver>> {
    let mut resolvers: Vec<Box<Resolver>> = Vec::new();

    if let Some(resolver) = setup_path_resolver(manifest, matches)? {
        resolvers.push(resolver);
    }

    resolvers.push(Box::new(setup_repository(matches)?));
    Ok(Box::new(Resolvers::new(resolvers)))
}

pub fn setup_options(manifest: &Manifest, matches: &ArgMatches) -> Result<Options> {
    let id_converter = if let Some(id_converter) = matches.value_of("id-converter") {
        Some(parse_id_converter(&id_converter)?)
    } else {
        None
    };

    let modules = matches
        .values_of("module")
        .into_iter()
        .flat_map(|it| it)
        .map(ToOwned::to_owned)
        .chain(manifest.modules.iter().cloned())
        .collect();

    Ok(Options {
        id_converter: id_converter,
        modules: modules,
    })
}

pub fn setup_packages(manifest: &Manifest, matches: &ArgMatches) -> Result<Vec<RpRequiredPackage>> {
    let mut packages = Vec::new();

    for package in &manifest.packages {
        packages.push(package.clone());
    }

    for package in matches.values_of("package").into_iter().flat_map(|it| it) {
        let parsed = RpRequiredPackage::parse(package);

        let parsed = parsed.chain_err(|| {
            format!("failed to parse --package argument: {}", package)
        })?;

        packages.push(parsed);
    }

    Ok(packages)
}

pub fn setup_environment(manifest: &Manifest, matches: &ArgMatches) -> Result<Environment> {
    let resolvers = setup_resolvers(manifest, matches)?;

    let package_prefix = matches.value_of("package-prefix").map(ToOwned::to_owned);

    let package_prefix = package_prefix.clone().map(|prefix| {
        RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect())
    });

    Ok(Environment::new(package_prefix, resolvers))
}

/// Read the manifest based on the current environment.
pub fn setup_manifest<'a>(matches: &ArgMatches<'a>) -> Result<Manifest> {
    let mut manifest = Manifest::new();

    let manifest_path = matches
        .value_of("manifest-path")
        .map::<Result<PathBuf>, _>(|p| Ok(Path::new(p).to_owned()))
        .unwrap_or_else(|| Ok(env::current_dir()?.join(MANIFEST_NAME)))?;

    if manifest_path.is_file() {
        debug!("reading manifest: {}", manifest_path.display());
        let reader = File::open(manifest_path.clone())?;
        read_manifest(&mut manifest, manifest_path, reader)?;
    }

    Ok(manifest)
}

pub fn setup_files<'a>(matches: &'a ArgMatches) -> Vec<PathBuf> {
    matches
        .values_of("file")
        .into_iter()
        .flat_map(|it| it)
        .map(Path::new)
        .map(ToOwned::to_owned)
        .collect()
}

pub fn setup_env(matches: &ArgMatches) -> Result<(Manifest, Environment)> {
    let manifest = setup_manifest(matches)?;
    let files = setup_files(matches);
    let packages = setup_packages(&manifest, matches)?;
    let mut env = setup_environment(&manifest, matches)?;

    let mut errors = Vec::new();

    for file in files {
        if let Err(e) = env.import_file(file) {
            errors.push(e.into());
        }
    }

    for package in packages {
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

    Ok((manifest, env))
}

pub fn options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.subcommand(compiler_base(compile::options()));
    let out = out.subcommand(compiler_base(doc::options()));
    let out = out.subcommand(verify::options());
    let out = out.subcommand(publish::options());
    let out = out.subcommand(update::options());
    let out = out.subcommand(repo::options());
    let out = out.subcommand(manifest::options());
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    match name {
        "compile" => self::compile::entry(matches),
        "doc" => self::doc::entry(matches),
        "verify" => self::verify::entry(matches),
        "publish" => self::publish::entry(matches),
        "update" => self::update::entry(matches),
        "repo" => self::repo::entry(matches),
        "manifest" => self::manifest::entry(matches),
        _ => Err(format!("No such command: {}", name).into()),
    }
}
