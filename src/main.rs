extern crate log;
extern crate reproto_core;
extern crate reproto_parser;
extern crate reproto_backend;
extern crate clap;
extern crate reproto;

use reproto::error_handling;
use reproto::errors::*;
use reproto::logger;
use reproto::ops;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn setup_opts<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("reproto")
        .version(VERSION)
        .arg(clap::Arg::with_name("debug").long("debug").short("D").help("Enable debug output"))
}

/// Configure logging
///
/// If debug (--debug) is specified, logging should be configured with LogLevelFilter::Debug.
fn setup_logger(matches: &clap::ArgMatches) -> Result<()> {
    let level: log::LogLevelFilter = match matches.is_present("debug") {
        true => log::LogLevelFilter::Debug,
        false => log::LogLevelFilter::Info,
    };

    logger::init(level)?;

    Ok(())
}

fn entry() -> Result<()> {
    let opts = setup_opts();
    let opts = ops::options(opts);
    let matches = opts.get_matches();
    setup_logger(&matches)?;
    ops::entry(&matches)?;
    Ok(())
}

fn compiler_entry() -> Result<()> {
    if let Err(e) = entry() {
        if !error_handling::handle_error(&e)? {
            return Err(e);
        }

        ::std::process::exit(1);
    }

    Ok(())
}

fn main() {
    if let Err(e) = compiler_entry() {
        error_handling::print_root_error(&e);
        ::std::process::exit(1);
    }

    ::std::process::exit(0);
}
