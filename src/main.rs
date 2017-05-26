extern crate clap;
extern crate reproto;
#[macro_use]
extern crate log;

use reproto::backend::models as m;
use reproto::backend;
use reproto::commands;
use reproto::errors::*;
use reproto::logger;
use reproto::parser;

static VERSION: &str = "0.0.5";

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

fn handle_backend_error(e: &backend::errors::Error) -> Result<()> {
    match *e {
        backend::errors::Error::Message(ref m) => {
            println!("<unknown>: {}", m);
        }
        backend::errors::Error::Pos(ref m, ref p) => {
            print_error(m, p)?;
        }
        backend::errors::Error::DeclMerge(ref m, ref a, ref b) => {
            print_error(m, a)?;
            print_error("previous declaration", b)?;
        }
        backend::errors::Error::FieldConflict(ref name, ref source, ref target) => {
            print_error(&format!("conflict in field `{}`", name), source)?;
            print_error("previous field", target)?;
        }
        backend::errors::Error::Error(ref e) => {
            println!("<unknown>: {}", e);
        }
    }

    Ok(())
}

fn print_error(m: &str, p: &m::Pos) -> Result<()> {
    let (line, lines, range) = parser::find_line(&p.0, (p.1, p.2))?;

    println!("{}:{}:{}-{}: {}:",
             p.0.display(),
             lines + 1,
             range.0,
             range.1,
             m);

    let line_no = format!("{:>3}", lines + 1);
    let diff = range.1 - range.0;

    let mut line_indicator = String::new();

    line_indicator.push_str(&::std::iter::repeat(" ")
        .take(line_no.len() + range.0 + 1)
        .collect::<String>());
    line_indicator.push_str(&::std::iter::repeat("^").take(diff).collect::<String>());

    println!("{}: {}", line_no, line);
    println!("{} - here", line_indicator);

    Ok(())
}

fn entry() -> Result<()> {
    let mut opts = setup_opts();

    // setup subcommands
    for subcommand in commands::commands() {
        opts = opts.subcommand(subcommand);
    }

    let matches = opts.get_matches();

    setup_logger(&matches)?;

    let (name, matches) = matches.subcommand();

    if let Some(matches) = matches {
        match name {
            "compile" => commands::compile(matches),
            "verify" => commands::verify(matches),
            _ => Err(format!("No such command: {}", name).into()),
        }
    } else {
        Err("No matching subcommand".into())
    }
}

fn compiler_entry() -> Result<()> {
    match entry() {
        Err(e) => {
            match *e.kind() {
                ErrorKind::BackendErrors(ref errors) => {
                    for e in errors {
                        handle_backend_error(e)?;
                    }
                }
                ErrorKind::BackendError(ref e) => {
                    handle_backend_error(e)?;
                }
                _ => {}
            }

            Err(e)
        }
        ok => ok,
    }
}

fn main() {
    match compiler_entry() {
        Err(e) => {
            error!("{}", e);

            for e in e.iter().skip(1) {
                error!("  caused by: {}", e);
            }

            if let Some(backtrace) = e.backtrace() {
                error!("  backtrace: {:?}", backtrace);
            }

            ::std::process::exit(1);
        }
        _ => {}
    };

    ::std::process::exit(0);
}
