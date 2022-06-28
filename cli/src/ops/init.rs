//! Initialize a new project.

use clap::{App, Arg, ArgMatches, SubCommand};
use reproto_core::errors::Result;
use reproto_core::Filesystem;
use std::path::Path;

pub fn options<'a>() -> App<'a> {
    let out = SubCommand::with_name("init").about("Initialize a new project");

    let out = out.arg(
        Arg::with_name("path")
            .long("path")
            .takes_value(true)
            .help("Path to initialize the new project in. Defaults to current."),
    );

    out
}

pub fn entry(fs: &dyn Filesystem, matches: &ArgMatches) -> Result<()> {
    let path = if let Ok(Some(path)) = matches.try_get_one::<String>("path") {
        Path::new(path).to_owned()
    } else {
        ::std::env::current_dir()?
    };

    let handle = fs.open_root(Some(&path))?;
    env::initialize(handle.as_ref())?;
    Ok(())
}
