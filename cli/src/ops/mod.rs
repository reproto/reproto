mod build;
mod check;
mod derive;
mod doc;
mod init;
mod language_server;
mod publish;
mod repo;
mod self_update;
mod update;
mod watch;

use crate::output::Output;
use clap::{App, Arg, ArgMatches};
use core::errors::Result;
use core::{Filesystem, Reporter};

pub fn base_args<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.arg(
        Arg::with_name("debug")
            .long("debug")
            .short("D")
            .help("Enable debug output"),
    );

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

pub fn options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.subcommand(build_args(build::options()));
    let out = out.subcommand(build_args(language_server::options()));
    let out = out.subcommand(build_args(doc::options()));
    let out = out.subcommand(build_args(watch::options()));
    let out = out.subcommand(base_args(check::options()));
    let out = out.subcommand(base_args(publish::options()));
    let out = out.subcommand(base_args(update::options()));
    let out = out.subcommand(base_args(self_update::options()));
    let out = out.subcommand(base_args(repo::options()));
    let out = out.subcommand(derive::options());
    let out = out.subcommand(init::options());
    out
}

/// Configure default logging.
///
/// If debug (--debug) is specified, logging should be configured with `LogLevelFilter::Debug`.
fn default_logging(matches: &ArgMatches, output: &dyn Output) -> Result<()> {
    let level = if matches.is_present("debug") {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    log::set_boxed_logger(output.logger())?;
    log::set_max_level(level);

    Ok(())
}

pub fn entry(
    fs: &dyn Filesystem,
    reporter: &mut dyn Reporter,
    matches: &ArgMatches,
    output: &dyn Output,
) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    // has custom log setup.
    if name == "language-server" {
        return language_server::entry(matches);
    }

    // setup default logger.
    default_logging(matches, output)?;

    match name {
        "build" => return build::entry(fs, reporter, matches),
        "check" => return check::entry(reporter, matches),
        "derive" => return derive::entry(reporter, matches),
        "doc" => return doc::entry(reporter, matches),
        "init" => return init::entry(fs, matches),
        "publish" => return publish::entry(reporter, matches),
        "repo" => return repo::entry(matches),
        "self-update" => return self_update::entry(matches),
        "update" => return update::entry(matches),
        "watch" => return watch::entry(fs, matches, output),
        _ => {}
    }

    Err(format!("No such command: {}", name).into())
}
