use errors::*;
use reproto_backend as backend;
use reproto_core as core;
use reproto_parser as parser;

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
    use self::core::errors::ErrorKind::*;

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

fn handle_backend_error(e: &backend::errors::ErrorKind) -> Result<bool> {
    use self::backend::errors::ErrorKind::*;

    let out = match *e {
        Pos(ref m, ref p) => {
            print_error(m, p)?;
            true
        }
        Core(ref e) => {
            return handle_core_error(e);
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

fn handle_parser_error(e: &parser::errors::ErrorKind) -> Result<bool> {
    use self::parser::errors::ErrorKind::*;
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

pub fn handle_error(e: &Error) -> Result<bool> {
    use errors::ErrorKind::*;

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
        Backend(ref e) => {
            return handle_backend_error(e);
        }
        _ => false,
    };

    Ok(out)
}

pub fn print_root_error(e: &Error) {
    error!("error: {}", e);

    for cause in e.iter().skip(1) {
        error!("  caused by: {}", cause);
    }

    if let Some(backtrace) = e.backtrace() {
        error!("backtrace: {:?}", backtrace);
    }
}
