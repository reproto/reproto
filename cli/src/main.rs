extern crate log;
extern crate clap;
extern crate reproto;
extern crate atty;
extern crate reproto_core as core;

use clap::{App, Arg, ArgMatches};
use core::Context;
use reproto::errors::*;
use reproto::ops;
use reproto::output;
use std::io;
use std::rc::Rc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn setup_opts<'a, 'b>() -> App<'a, 'b> {
    App::new("reproto")
        .version(VERSION)
        .arg(Arg::with_name("debug").long("debug").short("D").help(
            "Enable debug \
             output",
        ))
        .arg(Arg::with_name("color").long("color").help(
            "Force colored output",
        ))
        .arg(Arg::with_name("no-color").long("no-color").help(
            "Disable colored \
             output",
        ))
}

/// Configure logging
///
/// If debug (--debug) is specified, logging should be configured with `LogLevelFilter::Debug`.
fn setup_logger(matches: &clap::ArgMatches, output: &output::Output) -> Result<()> {
    let level = if matches.is_present("debug") {
        log::LogLevelFilter::Debug
    } else {
        log::LogLevelFilter::Info
    };

    log::set_logger(|max_level| {
        max_level.set(level);
        output.logger()
    })?;

    Ok(())
}

fn guarded_entry(ctx: Rc<Context>, matches: &ArgMatches, output: &output::Output) -> Result<()> {
    setup_logger(matches, output)?;
    ops::entry(ctx, matches)?;
    Ok(())
}

fn entry(matches: &ArgMatches, output: &output::Output) -> Result<()> {
    let ctx = Rc::new(Context::default());

    if let Err(e) = guarded_entry(Rc::clone(&ctx), matches, output) {
        output.handle_context(ctx.as_ref())?;

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

    if let Err(e) = entry(&matches, output.as_mut()) {
        output.print_root_error(&e).unwrap();
        ::std::process::exit(1);
    }

    ::std::process::exit(0);
}
