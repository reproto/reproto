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

fn print_error(m: &str, p: &m::Pos) -> Result<()> {
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

fn handle_backend_error(e: &backend::errors::ErrorKind) -> Result<()> {
    match *e {
        backend::errors::ErrorKind::Pos(ref m, ref p) => {
            print_error(m, p)?;
        }
        backend::errors::ErrorKind::DeclMerge(ref m, ref source, ref target) => {
            print_error(m, source)?;
            print_error("previous declaration here", target)?;
        }
        backend::errors::ErrorKind::FieldConflict(ref name, ref source, ref target) => {
            print_error(&format!("conflict in field `{}`", name), source)?;
            print_error("previous declaration here", target)?;
        }
        backend::errors::ErrorKind::ExtendEnum(ref m, ref source, ref enum_target) => {
            print_error(m, source)?;
            print_error("previous declaration here", enum_target)?;
        }
        backend::errors::ErrorKind::ReservedField(ref field_pos, ref reserved_pos) => {
            print_error("field reserved", field_pos)?;
            print_error("field reserved here", reserved_pos)?;
        }
        backend::errors::ErrorKind::MatchConflict(ref source, ref target) => {
            print_error("conflicts with existing clause", source)?;
            print_error("existing clause here", target)?;
        }
        backend::errors::ErrorKind::Parser(ref e) => {
            handle_parser_error(e)?;
        }
        _ => {}
    }

    Ok(())
}

fn handle_parser_error(e: &parser::errors::ErrorKind) -> Result<()> {
    match *e {
        parser::errors::ErrorKind::Syntax(ref p, ref expected) => {
            print_error("syntax error", p)?;

            println!("Expected one of:");

            let mut expected_list = Vec::new();

            for e in expected {
                match *e {
                    parser::parser::Rule::type_identifier => {
                        println!("  A type identifier, like: `DateRange`");
                    }
                    parser::parser::Rule::string => {
                        println!("  A string, like: `\"foo bar\"`");
                    }
                    parser::parser::Rule::number => {
                        println!("  A number number, like: `3.14`");
                    }
                    parser::parser::Rule::boolean => {
                        println!("  A boolean: `true` or `false`");
                    }
                    token => {
                        expected_list.push(format!("{:?}", token));
                    }
                }
            }

            if !expected_list.is_empty() {
                println!("  A token: {}", expected_list.join(", "));
            }
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
