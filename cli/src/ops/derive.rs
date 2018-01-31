//! Derive a schema from the given input.

use clap::{App, Arg, ArgMatches, SubCommand};
use core::{Context, PathObject, ReaderObject};
use derive;
use errors::Result;
use std::io;
use std::path::Path;
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("derive").about("Derive a schema from the given input");

    let out = out.arg(
        Arg::with_name("file")
            .long("file")
            .short("i")
            .takes_value(true)
            .help("File to read from, otherwise will read from stdin"),
    );

    out
}

pub fn entry(_ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    match matches.value_of("file") {
        Some(file) => {
            let object = PathObject::new(None, Path::new(file));
            derive::derive(object)?
        }
        None => {
            let reader = || Box::new(io::stdin()) as Box<io::Read>;
            let object = ReaderObject::new("<stdin>".to_string(), reader);

            derive::derive(object)?;
        }
    }

    Ok(())
}
