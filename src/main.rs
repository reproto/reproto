extern crate reproto_core;
extern crate reproto_parser;
extern crate clap;
extern crate reproto;
#[macro_use]
extern crate log;

use reproto::backend;
use reproto::commands;
use reproto::errors::*;
use reproto::logger;
use reproto_core as core;
use reproto_parser as parser;

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

fn print_error(m: &str, p: &core::RpPos) -> Result<()> {
    let (line, lines, range) = parser::find_line(&p.0, (p.1, p.2))?;

    println!("{}:{}:{}-{}:", p.0.display(), lines + 1, range.0, range.1);

    let line_no = format!("{:>3}", lines + 1);
    let diff = range.1 - range.0;
    let diff = if diff < 1 { 1 } else { diff };

    let mut line_indicator = String::new();

    line_indicator.push_str(&::std::iter::repeat(" ")
        .take(line_no.len() + range.0 + 1)
        .collect::<String>());
    line_indicator.push_str(&::std::iter::repeat("^").take(diff).collect::<String>());

    println!("{}: {}", line_no, line);
    println!("{} - {}", line_indicator, m);

    Ok(())
}

fn handle_core_error(e: &core::errors::ErrorKind) -> Result<()> {
    use core::errors::ErrorKind::*;

    match *e {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
        }
        DeclMerge(ref m, ref source, ref target) => {
            print_error(m, source)?;
            print_error("previous declaration here", target)?;
        }
        FieldConflict(ref name, ref source, ref target) => {
            print_error(&format!("conflict in field `{}`", name), source)?;
            print_error("previous declaration here", target)?;
        }
        ExtendEnum(ref m, ref source, ref enum_target) => {
            print_error(m, source)?;
            print_error("previous declaration here", enum_target)?;
        }
        ReservedField(ref field_pos, ref reserved_pos) => {
            print_error("field reserved", field_pos)?;
            print_error("field reserved here", reserved_pos)?;
        }
        MatchConflict(ref source, ref target) => {
            print_error("conflicts with existing clause", source)?;
            print_error("existing clause here", target)?;
        }
        _ => {}
    }

    Ok(())
}

fn handle_backend_error(e: &backend::errors::ErrorKind) -> Result<()> {
    use backend::errors::ErrorKind::*;

    match *e {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
        }
        Core(ref core) => {
            handle_core_error(core)?;
        }
        Parser(ref e) => {
            handle_parser_error(e)?;
        }
        MissingRequired(ref names, ref location, ref fields) => {
            print_error(&format!("missing required fields: {}", names.join(", ")),
                        location)?;

            for f in fields {
                print_error("required field", f)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_parser_error(e: &parser::errors::ErrorKind) -> Result<()> {
    use parser::errors::ErrorKind::*;

    match *e {
        Core(ref e) => {
            handle_core_error(e)?;
        }
        Syntax(ref p, ref expected) => {
            if let Some(ref pos) = *p {
                print_error("syntax error", pos)?;
            }

            if !expected.is_empty() {
                println!("Expected one of: {}", expected.join(", "));
            }
        }
        FieldConflict(ref name, ref source, ref target) => {
            print_error(&format!("conflict in field `{}`", name), source)?;
            print_error("previous declaration here", target)?;
        }
        EnumVariantConflict(ref pos, ref other) => {
            print_error("conflicting name", pos)?;
            print_error("previous name here", other)?;
        }
        _ => {}
    }

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
                ErrorKind::Parser(ref e) => {
                    handle_parser_error(e)?;
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
            match *e.kind() {
                ErrorKind::BackendErrors(ref errors) => {
                    for e in errors {
                        error!("error: {}", e);
                    }
                }
                _ => {
                    error!("error: {}", e);
                }
            }

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
