//! Derive a schema from the given input.

use clap::{App, Arg, ArgMatches, SubCommand};
use core::{Context, Object, PathObject, StdinObject};
use core::errors::Result;
use derive;
use genco::{IoFmt, WriteTokens};
use reproto;
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

    let out = out.arg(
        Arg::with_name("format")
            .long("format")
            .short("F")
            .takes_value(true)
            .help("Format to decode, valid values: json, yaml"),
    );

    out
}

pub fn entry(_ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let format: Box<derive::Format> = match matches.value_of("format") {
        None | Some("json") => Box::new(derive::Json),
        Some("yaml") => Box::new(derive::Yaml),
        Some(value) => return Err(format!("Unsupported format: {}", value).into()),
    };

    let object: Box<Object> = match matches.value_of("file") {
        Some(file) => Box::new(PathObject::new(None, Path::new(file))),
        None => Box::new(StdinObject::new()),
    };

    let decl = derive::derive(format, object.as_ref())?;

    let stdout = io::stdout();
    let toks = reproto::format(&decl)?;

    IoFmt(&mut stdout.lock()).write_file(toks, &mut ())?;

    Ok(())
}
