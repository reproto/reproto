#[macro_use]
extern crate log;
extern crate reproto_core;
extern crate reproto_parser;
extern crate clap;
extern crate reproto;
extern crate ansi_term;

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

fn print_error<S: AsRef<str>>(m: S, p: &core::ErrorPos) -> Result<()> {
    use std::iter::repeat;
    use std::cmp::max;
    use ansi_term::Colour::{Blue, Red};

    let (line_str, line, (s, e)) = parser::find_line(&p.path, (p.start, p.end))?;

    println!("{}:{}:{}-{}:", p.path.display(), line + 1, s + 1, e + 1);

    let line_no = format!("{:>3}:", line + 1);

    let mut indicator = String::new();

    indicator.extend(repeat(' ').take(line_no.len() + s + 1));
    indicator.extend(repeat('^').take(max(1, e - s)));

    println!("{} {}", Blue.paint(line_no), line_str);
    println!("{}{}{}",
             Red.paint(indicator),
             Red.paint(" - "),
             Red.paint(m.as_ref()));

    Ok(())
}

fn handle_core_error(e: &core::errors::ErrorKind) -> Result<bool> {
    use core::errors::ErrorKind::*;

    let out = match *e {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
            true
        }
        DeclMerge(ref m, ref source, ref target) => {
            print_error(m, source)?;
            print_error("previous declaration here", target)?;
            true
        }
        FieldConflict(ref name, ref source, ref target) => {
            print_error(&format!("conflict in field `{}`", name), source)?;
            print_error("previous declaration here", target)?;
            true
        }
        ExtendEnum(ref m, ref source, ref enum_target) => {
            print_error(m, source)?;
            print_error("previous declaration here", enum_target)?;
            true
        }
        ReservedField(ref field_pos, ref reserved_pos) => {
            print_error("field reserved", field_pos)?;
            print_error("field reserved here", reserved_pos)?;
            true
        }
        MatchConflict(ref source, ref target) => {
            print_error("conflicts with existing clause", source)?;
            print_error("existing clause here", target)?;
            true
        }
        _ => false,
    };

    Ok(out)
}

fn handle_parser_error(e: &parser::errors::ErrorKind) -> Result<bool> {
    use parser::errors::ErrorKind::*;
    use ansi_term::Colour::Red;

    let out = match *e {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
            true
        }
        Core(ref e) => {
            return handle_core_error(e);
        }
        Syntax(ref p, ref expected) => {
            let m = if !expected.is_empty() {
                format!("unexpected token, expected one of: {}", expected.join(", "))
            } else {
                String::from("syntax error")
            };

            if let Some(ref pos) = *p {
                print_error(m, pos)?;
            } else {
                println!("{}", Red.paint(m));
            }

            true
        }
        Parse(ref message, ref pos) => {
            print_error(message, pos)?;
            true
        }
        FieldConflict(ref name, ref source, ref target) => {
            print_error(&format!("conflict in field `{}`", name), source)?;
            print_error("previous declaration here", target)?;
            true
        }
        EnumVariantConflict(ref pos, ref other) => {
            print_error("conflicting name", pos)?;
            print_error("previous name here", other)?;
            true
        }
        _ => false,
    };

    Ok(out)
}

fn handle_error(e: &Error) -> Result<bool> {
    use reproto::errors::ErrorKind::*;

    let out = match *e.kind() {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
            true
        }
        Errors(ref errors) => {
            for e in errors {
                if !handle_error(e)? {
                    print_root_error(e);
                }
            }

            true
        }
        Core(ref core) => {
            return handle_core_error(core);
        }
        Parser(ref e) => {
            return handle_parser_error(e);
        }
        MissingRequired(ref names, ref location, ref fields) => {
            print_error(&format!("missing required fields: {}", names.join(", ")),
                        location)?;

            for f in fields {
                print_error("required field", f)?;
            }

            true
        }
        _ => false,
    };

    Ok(out)
}

fn entry() -> Result<()> {
    let opts = setup_opts();
    let opts = commands::commands(opts);
    let matches = opts.get_matches();
    setup_logger(&matches)?;

    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    match name {
        "compile" => commands::compile(matches),
        "verify" => commands::verify(matches),
        _ => Err(format!("No such command: {}", name).into()),
    }
}

fn print_root_error(e: &Error) {
    error!("error: {}", e);

    for cause in e.iter().skip(1) {
        error!("  caused by: {}", cause);
    }

    if let Some(backtrace) = e.backtrace() {
        error!("backtrace: {:?}", backtrace);
    }
}

fn compiler_entry() -> Result<()> {
    if let Err(e) = entry() {
        if !handle_error(&e)? {
            return Err(e);
        }

        ::std::process::exit(1);
    }

    Ok(())
}

fn main() {
    if let Err(e) = compiler_entry() {
        print_root_error(&e);
        ::std::process::exit(1);
    }

    ::std::process::exit(0);
}
