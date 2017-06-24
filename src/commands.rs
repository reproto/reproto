use backend::{self, CompilerOptions, Environment};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::{RpPackage, RpRequiredPackage, VersionReq};
use errors::*;
use naming;
use options::Options;
use reproto_repository::{Filesystem, Paths, Resolver, Resolvers};
use std::env;
use std::error::Error;
use std::path::Path;

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

pub struct CompileOptions {
}

pub fn compile_options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("compile").about("Compile .reproto specifications");
    let out = out.subcommand(backend::doc::compile_options(compile_base("doc")));
    let out = out.subcommand(backend::java::compile_options(compile_base("java")));
    let out = out.subcommand(backend::js::compile_options(compile_base("js")));
    let out = out.subcommand(backend::python::compile_options(compile_base("python")));
    let out = out.subcommand(backend::rust::compile_options(compile_base("rust")));
    out
}

pub fn verify_options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("verify").about("Verify .reproto specifications");
    let out = out.subcommand(backend::doc::verify_options(shared_base("doc")));
    let out = out.subcommand(backend::java::verify_options(shared_base("java")));
    let out = out.subcommand(backend::js::verify_options(shared_base("js")));
    let out = out.subcommand(backend::python::verify_options(shared_base("python")));
    let out = out.subcommand(backend::rust::verify_options(shared_base("rust")));
    out
}

pub fn compile_base<'a, 'b>(name: &str) -> App<'a, 'b> {
    let out = shared_base(name).about("Compile .reproto specifications");

    let out = out.arg(Arg::with_name("out")
        .long("out")
        .short("o")
        .takes_value(true)
        .help("Output directory."));

    out
}

pub fn shared_base<'a, 'b>(name: &str) -> App<'a, 'b> {
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

fn setup_resolvers(matches: &ArgMatches) -> Box<Resolver> {
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
        let repository = home_dir.join(".reproto").join("repository");

        if repository.is_dir() {
            debug!("using repository: {}", repository.display());
            resolvers.push(Box::new(Filesystem::new(repository)));
        }
    }

    Box::new(Resolvers::new(resolvers))
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

fn setup_environment(matches: &ArgMatches) -> Environment {
    let resolvers = setup_resolvers(matches);

    let package_prefix = matches.value_of("package-prefix").map(ToOwned::to_owned);

    let package_prefix = package_prefix.clone()
        .map(|prefix| RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    Environment::new(package_prefix, resolvers)
}

fn setup_files<'a>(matches: &'a ArgMatches) -> Vec<&'a Path> {
    matches.values_of("file").into_iter().flat_map(|it| it).map(Path::new).collect()
}

fn setup_env(matches: &ArgMatches) -> Result<Environment> {
    let files = setup_files(matches);
    let packages = setup_packages(matches)?;
    let mut env = setup_environment(matches);

    let mut errors = Vec::new();

    for file in files {
        if let Err(e) = env.import_file(file) {
            errors.push(e);
        }
    }

    for package in packages {
        if let Err(e) = env.import(&package) {
            errors.push(e);
        }
    }

    if let Err(e) = env.verify() {
        errors.push(e);
    }

    if !errors.is_empty() {
        return Err(ErrorKind::Errors(errors).into());
    }

    Ok(env)
}

fn setup_compiler_options(matches: &ArgMatches) -> Result<CompilerOptions> {
    let out_path = matches.value_of("out").ok_or("--out <dir> is required")?;
    let out_path = Path::new(&out_path);

    Ok(CompilerOptions { out_path: out_path.to_owned() })
}

pub fn compile(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    let env = setup_env(matches)?;
    let options = setup_options(matches)?;
    let compiler_options = setup_compiler_options(matches)?;

    match name {
        "doc" => backend::doc::compile(env, options, compiler_options, matches),
        "java" => backend::java::compile(env, options, compiler_options, matches),
        "js" => backend::js::compile(env, options, compiler_options, matches),
        "json" => backend::json::compile(env, options, compiler_options, matches),
        "python" => backend::python::compile(env, options, compiler_options, matches),
        "rust" => backend::rust::compile(env, options, compiler_options, matches),
        _ => unreachable!("bad subcommand"),
    }
}

pub fn verify(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    let env = setup_env(matches)?;
    let options = setup_options(matches)?;

    match name {
        "doc" => backend::doc::verify(env, options, matches),
        "java" => backend::java::verify(env, options, matches),
        "js" => backend::js::verify(env, options, matches),
        "json" => backend::json::verify(env, options, matches),
        "python" => backend::python::verify(env, options, matches),
        "rust" => backend::rust::verify(env, options, matches),
        _ => unreachable!("bad subcommand"),
    }
}

pub fn commands<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.subcommand(compile_options());
    let out = out.subcommand(verify_options());
    out
}
