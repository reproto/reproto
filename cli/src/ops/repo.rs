//! Repository management commands.

use clap::{App, Arg, ArgMatches, SubCommand};
use repository::init_file_index;
use reproto_core::errors::Result;

fn init(matches: &ArgMatches) -> Result<()> {
    if let Ok(Some(paths)) = matches.try_get_many::<String>("path") {
        for path in paths {
            log::info!("Creating repository: {}", path);
            init_file_index(path)?;
        }
    }

    Ok(())
}

fn init_options<'a>() -> App<'a> {
    let out = SubCommand::with_name("init").about("Initialize a new repository");

    let out = out.arg(
        Arg::with_name("path")
            .required(true)
            .help("Path to repository to initialize"),
    );

    out
}

pub fn options<'a>() -> App<'a> {
    let out = SubCommand::with_name("repo").about("Manage repositories");
    let out = out.subcommand(init_options());
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand().ok_or_else(|| "no subcommand")?;

    match name {
        "init" => init(matches),
        _ => unreachable!("bad subcommand"),
    }
}
