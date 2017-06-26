pub mod verify;
pub mod compile;

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
use url::Url;

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

/// Setup base compiler options.
pub fn compiler_base<'a, 'b>(name: &str) -> App<'a, 'b> {
    SubCommand::with_name(name)
        .arg(Arg::with_name("module")
            .long("module")
            .short("m")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1)
            .help("Modules to load for a given backend"))
        .arg(Arg::with_name("path")
            .long("path")
            .short("p")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1)
            .help("Paths to look for definitions."))
        .arg(Arg::with_name("id-converter")
            .long("id-converter")
            .takes_value(true)
            .help("Conversion method to use when naming fields by default"))
        .arg(Arg::with_name("package-prefix")
            .long("package-prefix")
            .takes_value(true)
            .help("Package prefix to use when generating classes"))
        .arg(Arg::with_name("file")
            .long("file")
            .help("File to compile")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1))
        .arg(Arg::with_name("package")
            .long("package")
            .help("Packages to compile")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1))
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

fn setup_resolvers(matches: &ArgMatches) -> Result<Box<Resolver>> {
    let mut resolvers: Vec<Box<Resolver>> = Vec::new();

    let paths: Vec<::std::path::PathBuf> = matches.values_of("path")
        .into_iter()
        .flat_map(|it| it)
        .map(Path::new)
        .map(ToOwned::to_owned)
        .collect();

    if !paths.is_empty() {
        resolvers.push(Box::new(Paths::new(paths)));
    }

    if let Some(home_dir) = env::home_dir() {
        let reproto_dir = home_dir.join(".reproto");
        let config = reproto_dir.join("config.toml");

        if config.is_file() {
            let config = read_config(config)?;

            if let Some(ref index_url) = config.repository.index {
                let index_url = Url::parse(index_url)?;
                let index = index_from_url(&index_url)?;

                let objects_url = if let Some(ref objects) = config.repository.objects {
                    Url::parse(objects)?
                } else {
                    index.objects_url()?
                };

                debug!("index: {}", index_url);
                debug!("objects: {}", objects_url);

                let objects = objects_from_url(&objects_url)?;
                let repository = Repository::new(index, objects);

                resolvers.push(Box::new(repository));
            }
        }
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
    out
}
