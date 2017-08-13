extern crate log;
extern crate reproto_core;
extern crate reproto_parser;
extern crate reproto_backend;
extern crate clap;
extern crate reproto;
extern crate atty;

use clap::{App, Arg, ArgMatches};
use reproto::errors::*;
use reproto::ops;
use reproto::output;
use std::io;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn setup_opts<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("reproto").version(VERSION);
    let app = app.arg(Arg::with_name("debug").long("debug").short("D").help(
        "Enable debug \
         output",
    ));
    let app = app.arg(Arg::with_name("color").long("color").help(
        "Force colored output",
    ));
    let app = app.arg(Arg::with_name("no-color").long("no-color").help(
        "Disable colored \
         output",
    ));
    app
}

/// Configure logging
///
/// If debug (--debug) is specified, logging should be configured with LogLevelFilter::Debug.
fn setup_logger(matches: &clap::ArgMatches, output: &output::Output) -> Result<()> {
    let level: log::LogLevelFilter = match matches.is_present("debug") {
        true => log::LogLevelFilter::Debug,
        false => log::LogLevelFilter::Info,
    };

    log::set_logger(|max_level| {
        max_level.set(level);
        output.logger()
    })?;

    Ok(())
}

fn entry(matches: ArgMatches, output: &output::Output) -> Result<()> {
    setup_logger(&matches, output)?;
    ops::entry(&matches)?;
    Ok(())
}

fn compiler_entry(matches: ArgMatches, output: &output::Output) -> Result<()> {
    if let Err(e) = entry(matches, output) {
        if !output.handle_error(&e)? {
            return Err(e);
        }

        ::std::process::exit(1);
    }

    Ok(())
}

fn main() {
    let opts = setup_opts();
    let opts = ops::options(opts);
    let matches = opts.get_matches();

    let colored = matches.is_present("color") ||
        !matches.is_present("no-color") && atty::is(atty::Stream::Stdout);

    let mut output: Box<output::Output> = if colored {
        Box::new(output::Colored::new(io::stdout()))
    } else {
        Box::new(output::NonColored::new(io::stdout()))
    };

    if let Err(e) = compiler_entry(matches, output.as_mut()) {
        output.print_root_error(&e).unwrap();
        ::std::process::exit(1);
    }

    ::std::process::exit(0);
}
