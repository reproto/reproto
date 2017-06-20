extern crate reproto_core;
extern crate reproto_parser;
extern crate clap;
extern crate reproto;
#[macro_use]
extern crate log;

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

    let out = match *e {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
            true
        }
        Core(ref e) => {
            return handle_core_error(e);
        }
        Syntax(ref p, ref expected) => {
            if let Some(ref pos) = *p {
                print_error("syntax error", pos)?;
            }

            if !expected.is_empty() {
                println!("Expected one of: {}", expected.join(", "));
            }

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
        handle_error(&e)?;
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
