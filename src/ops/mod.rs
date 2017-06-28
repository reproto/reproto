pub mod verify;
pub mod compile;
pub mod publish;
pub mod update;

use reproto_backend_doc as doc;
use reproto_backend_java as java;
use reproto_backend_js as js;
use reproto_backend_json as json;
use reproto_backend_python as python;
use reproto_backend_rust as rust;
use reproto_repository::*;
use std::env;
use std::error::Error;
use std::path::Path;
use super::*;
use url;

fn parse_id_converter(input: &str) -> Result<Box<naming::Naming>> {
    let mut parts = input.split(":");

    if let Some(first) = parts.next() {
        if let Some(second) = parts.next() {
            let naming: Box<naming::FromNaming> = match first {
                "camel" => Box::new(naming::CamelCase::new()),
                "snake" => Box::new(naming::SnakeCase::new()),
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

    return Err(format!("Invalid --id-conversion argument: {}", input).into());
}

pub fn path_base<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.arg(Arg::with_name("index")
        .long("index")
        .short("I")
        .takes_value(true)
        .help("URL for index to use when looking up packages."));

    let out = out.arg(Arg::with_name("objects")
        .long("objects")
        .short("O")
        .takes_value(true)
        .help("URL for objects storage to use when looking up packages."));

    let out = out.arg(Arg::with_name("path")
        .long("path")
        .short("p")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .help("Paths to look for definitions."));

    out
}

/// Setup base compiler options.
pub fn compiler_base<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = path_base(out);

    let out = out.arg(Arg::with_name("package")
        .long("package")
        .help("Packages to compile")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1));

    let out = out.arg(Arg::with_name("module")
        .long("module")
        .short("m")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .help("Modules to load for a given backend"));

    let out = out.arg(Arg::with_name("id-converter")
        .long("id-converter")
        .takes_value(true)
        .help("Conversion method to use when naming fields by default"));

    let out = out.arg(Arg::with_name("package-prefix")
        .long("package-prefix")
        .takes_value(true)
        .help("Package prefix to use when generating classes"));

    let out = out.arg(Arg::with_name("file")
        .long("file")
        .help("File to compile")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1));

    out
}

fn parse_package(input: &str) -> Result<RpRequiredPackage> {
    let mut it = input.split("@").into_iter();

    let package = if let Some(first) = it.next() {
        RpPackage::new(first.split(".").map(ToOwned::to_owned).collect())
    } else {
        RpPackage::new(vec![])
    };

    let version_req = if let Some(version) = it.next() {
        Some(VersionReq::parse(version).map_err(|e| e.description().to_owned())?)
    } else {
        None
    };

    Ok(RpRequiredPackage::new(package, version_req))
}

fn setup_repository(matches: &ArgMatches) -> Result<Option<Repository>> {
    let mut index = matches.value_of("index").map(ToOwned::to_owned);
    let mut objects = matches.value_of("objects").map(ToOwned::to_owned);
    let mut index_config = IndexConfig { repos: None };
    let mut objects_config = ObjectsConfig { repos: None };

    if let Some(home_dir) = env::home_dir() {
        let reproto_dir = home_dir.join(".reproto");
        let config = reproto_dir.join("config.toml");
        let reproto_dir = home_dir.join(".reproto");
        let default_local_repos = reproto_dir.join("git");

        if config.is_file() {
            let config = read_config(config)?;

            // set values from configuration (if not already set).
            index = index.or(config.repository.index);
            objects = objects.or(config.repository.objects);

            let local_repos = config.repository.local_repos;

            index_config.repos = index_config.repos.or_else(|| local_repos.clone());
            objects_config.repos = objects_config.repos.or_else(|| local_repos.clone());
        }

        index_config.repos = Some(index_config.repos
            .unwrap_or_else(|| default_local_repos.clone()));
        objects_config.repos = Some(objects_config.repos
            .unwrap_or_else(|| default_local_repos.clone()));
    }

    if let Some(ref index_url) = index {
        let index_url = url::Url::parse(index_url)?;
        let index = index_from_url(index_config, &index_url)?;

        let objects_url = if let Some(objects) = objects {
            objects
        } else {
            index.objects_url()?
        };

        let objects = match url::Url::parse(&objects_url) {
            /// Relative to index index repository!
            Err(url::ParseError::RelativeUrlWithoutBase) => {
                let relative_path = Path::new(&objects_url);
                index.objects_from_index(&relative_path)?
            }
            Err(e) => return Err(e.into()),
            Ok(url) => objects_from_url(objects_config, &url)?,
        };

        let repository = Repository::new(index, objects);

        debug!("index: {}", index_url);
        debug!("objects: {}", objects_url);

        return Ok(Some(repository));
    }

    Ok(None)
}

fn setup_path_resolver(matches: &ArgMatches) -> Result<Option<Box<Resolver>>> {
    let paths: Vec<::std::path::PathBuf> = matches.values_of("path")
        .into_iter()
        .flat_map(|it| it)
        .map(Path::new)
        .map(ToOwned::to_owned)
        .collect();

    if paths.is_empty() {
        return Ok(None);
    }

    Ok(Some(Box::new(Paths::new(paths))))
}

fn setup_resolvers(matches: &ArgMatches) -> Result<Box<Resolver>> {
    let mut resolvers: Vec<Box<Resolver>> = Vec::new();

    if let Some(resolver) = setup_path_resolver(matches)? {
        resolvers.push(resolver);
    }

    if let Some(repository) = setup_repository(matches)? {
        resolvers.push(Box::new(repository));
    }

    Ok(Box::new(Resolvers::new(resolvers)))
}

fn setup_options(matches: &ArgMatches) -> Result<Options> {
    let id_converter = if let Some(id_converter) = matches.value_of("id-converter") {
        Some(parse_id_converter(&id_converter)?)
    } else {
        None
    };

    let modules =
        matches.values_of("module").into_iter().flat_map(|it| it).map(ToOwned::to_owned).collect();

    Ok(Options {
        id_converter: id_converter,
        modules: modules,
    })
}

fn setup_packages(matches: &ArgMatches) -> Result<Vec<RpRequiredPackage>> {
    let mut packages = Vec::new();

    for package in matches.values_of("package").into_iter().flat_map(|it| it) {
        let parsed = parse_package(package);
        let parsed =
            parsed.chain_err(|| format!("failed to parse --package argument: {}", package))?;
        packages.push(parsed);
    }

    Ok(packages)
}

fn setup_environment(matches: &ArgMatches) -> Result<Environment> {
    let resolvers = setup_resolvers(matches)?;

    let package_prefix = matches.value_of("package-prefix").map(ToOwned::to_owned);

    let package_prefix = package_prefix.clone()
        .map(|prefix| RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    Ok(Environment::new(package_prefix, resolvers))
}

fn setup_files<'a>(matches: &'a ArgMatches) -> Vec<&'a Path> {
    matches.values_of("file").into_iter().flat_map(|it| it).map(Path::new).collect()
}

fn setup_env(matches: &ArgMatches) -> Result<Environment> {
    let files = setup_files(matches);
    let packages = setup_packages(matches)?;
    let mut env = setup_environment(matches)?;

    let mut errors = Vec::new();

    for file in files {
        if let Err(e) = env.import_file(file) {
            errors.push(e.into());
        }
    }

    for package in packages {
        if let Err(e) = env.import(&package) {
            errors.push(e.into());
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
    let out = out.subcommand(compile::options());
    let out = out.subcommand(verify::options());
    let out = out.subcommand(publish::options());
    let out = out.subcommand(update::options());
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    match name {
        "compile" => ops::compile::entry(matches),
        "verify" => ops::verify::entry(matches),
        "publish" => ops::publish::entry(matches),
        "update" => ops::update::entry(matches),
        _ => Err(format!("No such command: {}", name).into()),
    }
}
