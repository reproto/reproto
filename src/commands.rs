use backend;
use backend::environment::Environment;
use clap::{App, Arg, ArgMatches, SubCommand};
use core::*;
use errors::*;
use naming;
use options::Options;
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

pub fn compile_options<'a, 'b>(name: &str) -> App<'a, 'b> {
    SubCommand::with_name(name)
        .arg(Arg::with_name("backend")
            .long("backend")
            .short("b")
            .help("Backend to used to emit code")
            .takes_value(true)
            .required(true))
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
        .arg(Arg::with_name("out")
            .long("out")
            .short("o")
            .takes_value(true)
            .help("Output directory."))
        .arg(Arg::with_name("id-converter")
            .long("id-converter")
            .takes_value(true)
            .help("Convert arguments"))
        .arg(Arg::with_name("package-prefix")
            .long("package-prefix")
            .takes_value(true)
            .help("RpPackage prefix to use when generating classes"))
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

fn setup_compiler<'a>(matches: &'a ArgMatches)
                      -> Result<(Vec<&'a Path>, Vec<RpPackage>, Environment, Options, &'a str)> {
    let paths: Vec<::std::path::PathBuf> = matches.values_of("path")
        .into_iter()
        .flat_map(|it| it)
        .map(Path::new)
        .map(ToOwned::to_owned)
        .collect();

    let backend = matches.value_of("backend").ok_or("--backend <backend> is required")?;
    let out_path = matches.value_of("out").ok_or("--out <dir> is required")?;

    let out_path = Path::new(&out_path);

    let package_prefix = matches.value_of("package-prefix").map(ToOwned::to_owned);

    let modules =
        matches.values_of("module").into_iter().flat_map(|it| it).map(ToOwned::to_owned).collect();

    let id_converter = if let Some(id_converter) = matches.value_of("id-converter") {
        Some(parse_id_converter(&id_converter)?)
    } else {
        None
    };

    let options = Options {
        out_path: out_path.to_path_buf(),
        package_prefix: package_prefix,
        id_converter: id_converter,
        modules: modules,
    };

    let env = Environment::new(paths);

    let files: Vec<&Path> = matches.values_of("file")
        .into_iter()
        .flat_map(|it| it)
        .map(Path::new)
        .collect();

    let packages: Vec<RpPackage> = matches.values_of("package")
        .into_iter()
        .flat_map(|it| it)
        .map(|s| RpPackage::new(s.split(".").map(ToOwned::to_owned).collect()))
        .collect();

    Ok((files, packages, env, options, backend))
}

fn do_compile(matches: &ArgMatches) -> Result<Box<backend::Backend>> {
    let (files, packages, mut env, options, backend) = setup_compiler(matches)?;

    let mut failed: Vec<backend::errors::Error> = Vec::new();

    for file in files {
        if let Err(e) = env.import_file(file, None) {
            failed.push(e);
        }
    }

    for package in packages {
        if let Err(e) = env.import(&package) {
            failed.push(e);
        }
    }

    if let Err(e) = env.verify() {
        failed.push(e);
    }

    let backend = backend::resolve(&backend, options, env);

    match backend {
        Err(e) => {
            failed.push(e);
            Err(failed.into())
        }
        Ok(backend) => {
            if failed.is_empty() {
                Ok(backend)
            } else {
                Err(failed.into())
            }
        }
    }
}

pub fn compile(matches: &ArgMatches) -> Result<()> {
    let backend = do_compile(matches)?;
    backend.process()?;
    Ok(())
}

pub fn verify(matches: &ArgMatches) -> Result<()> {
    let backend = do_compile(matches)?;

    let errors = backend.verify()?;

    if errors.is_empty() {
        return Ok(());
    }

    Err(ErrorKind::BackendErrors(errors).into())
}

pub fn commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    let mut commands = Vec::new();
    commands.push(compile_options("compile").about("Compile .reproto declarations"));
    commands.push(compile_options("verify").about("Verify .reproto declarations"));
    commands
}
