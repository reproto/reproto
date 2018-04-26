//! Initialize a new project.

use clap::{App, Arg, ArgMatches, SubCommand};
use core::Context;
use core::errors::*;
use env;
use std::path::Path;
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("init").about("Initialize a new project");

    let out = out.arg(
        Arg::with_name("path")
            .long("path")
            .takes_value(true)
            .help("Path to initialize the new project in. Defaults to current."),
    );

    out
}

pub fn entry(ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let path = if let Some(path) = matches.value_of("path") {
        Path::new(path).to_owned()
    } else {
        ::std::env::current_dir()?
    };

    let handle = ctx.filesystem(Some(&path))?;
    env::initialize(handle.as_ref())?;
    Ok(())
}
