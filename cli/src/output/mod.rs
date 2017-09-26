mod colored;
mod non_colored;

pub use self::colored::Colored;
pub use self::non_colored::NonColored;
use backend;
use core;
use errors::*;
use log;
use parser;
use repository;
use std::io::{self, Read, Write};

pub trait LockableWrite
where
    Self: Sync + Send,
{
    fn open_new(&self) -> Self;

    fn lock<'a>(&'a self) -> Box<Write + 'a>;
}

impl LockableWrite for io::Stdout {
    fn open_new(&self) -> Self {
        io::stdout()
    }

    fn lock<'a>(&'a self) -> Box<Write + 'a> {
        Box::new(self.lock())
    }
}

const NL: u8 = '\n' as u8;

fn find_line<'a, R: AsMut<Read + 'a>>(
    mut reader: R,
    pos: (usize, usize),
) -> Result<(String, usize, (usize, usize))> {
    let r = reader.as_mut();

    let mut line = 0usize;
    let mut current = 0usize;
    let mut buffer: Vec<u8> = Vec::new();

    let start = pos.0;
    let end = pos.1;

    let mut it = r.bytes().peekable();
    let mut read = 0usize;

    while let Some(b) = it.next() {
        let b = b?;
        read += 1;

        match b {
            NL => {}
            _ => {
                buffer.push(b);
                continue;
            }
        }

        let start_of_line = current;
        current += read;

        if current > start {
            let buffer = String::from_utf8(buffer)?;
            let end = ::std::cmp::min(end, current);
            let range = (start - start_of_line, end - start_of_line);
            return Ok((buffer, line, range));
        }

        read = 0usize;
        line += 1;
        buffer.clear();
    }

    Err("bad file position".into())
}

pub trait Output {
    fn handle_core_error(&self, e: &core::errors::ErrorKind) -> Result<bool> {
        use self::core::errors::ErrorKind::*;

        let out = match *e {
            Pos(ref m, ref p) => {
                self.print_error(m, p)?;
                true
            }
            DeclMerge(ref m, ref source, ref target) => {
                self.print_error(m, source)?;
                self.print_error("previous declaration here", target)?;
                true
            }
            FieldConflict(ref name, ref source, ref target) => {
                self.print_error(
                    &format!("conflict in field `{}`", name),
                    source,
                )?;
                self.print_error("previous declaration here", target)?;
                true
            }
            ExtendEnum(ref m, ref source, ref enum_target) => {
                self.print_error(m, source)?;
                self.print_error("previous declaration here", enum_target)?;
                true
            }
            ReservedField(ref field_pos, ref reserved_pos) => {
                self.print_error("field reserved", field_pos)?;
                self.print_error("field reserved here", reserved_pos)?;
                true
            }
            MatchConflict(ref source, ref target) => {
                self.print_error("conflicts with existing clause", source)?;
                self.print_error("existing clause here", target)?;
                true
            }
            _ => false,
        };

        Ok(out)
    }

    fn handle_backend_error(&self, e: &backend::errors::ErrorKind) -> Result<bool> {
        use self::backend::errors::ErrorKind::*;

        let out = match *e {
            Pos(ref m, ref p) => {
                self.print_error(m, p)?;
                true
            }
            Core(ref e) => {
                return self.handle_core_error(e);
            }
            Parser(ref e) => {
                return self.handle_parser_error(e);
            }
            MissingRequired(ref names, ref location, ref fields) => {
                self.print_error(
                    &format!(
                        "missing required fields: {}",
                        names.join(", ")
                    ),
                    location,
                )?;

                for f in fields {
                    self.print_error("required field", f)?;
                }

                true
            }
            FieldConflict(ref name, ref source, ref target) => {
                self.print_error(
                    &format!("conflict in field `{}`", name),
                    source,
                )?;
                self.print_error("previous declaration here", target)?;
                true
            }
            EnumVariantConflict(ref pos, ref other) => {
                self.print_error("conflicting name", pos)?;
                self.print_error("previous name here", other)?;
                true
            }
            _ => false,
        };

        Ok(out)
    }

    fn handle_parser_error(&self, e: &parser::errors::ErrorKind) -> Result<bool> {
        use self::parser::errors::ErrorKind::*;

        let out = match *e {
            Pos(ref m, ref p) => {
                self.print_error(m, p)?;
                true
            }
            Core(ref e) => {
                return self.handle_core_error(e);
            }
            Syntax(ref p, ref expected) => {
                let m = if !expected.is_empty() {
                    format!("unexpected token, expected one of: {}", expected.join(", "))
                } else {
                    String::from("syntax error")
                };

                if let Some(ref pos) = *p {
                    self.print_error(m.as_ref(), pos)?;
                } else {
                    self.print(m.as_ref())?;
                }

                true
            }
            Parse(ref message, ref pos) => {
                self.print_error(message, pos)?;
                true
            }
            _ => false,
        };

        Ok(out)
    }

    fn handle_repository_error(&self, e: &repository::errors::ErrorKind) -> Result<bool> {
        use self::repository::errors::ErrorKind::*;

        let out = match *e {
            Core(ref e) => {
                return self.handle_core_error(e);
            }
            _ => false,
        };

        Ok(out)
    }

    fn handle_error(&self, e: &Error) -> Result<bool> {
        use errors::ErrorKind::*;

        let out = match *e.kind() {
            Pos(ref m, ref p) => {
                self.print_error(m, p)?;
                true
            }
            Errors(ref errors) => {
                for e in errors {
                    if !self.handle_error(e)? {
                        self.print_root_error(e)?;
                    }
                }

                true
            }
            Core(ref core) => {
                return self.handle_core_error(core);
            }
            Parser(ref e) => {
                return self.handle_parser_error(e);
            }
            Backend(ref e) => {
                return self.handle_backend_error(e);
            }
            Repository(ref e) => {
                return self.handle_repository_error(e);
            }
            _ => false,
        };

        Ok(out)
    }

    fn logger(&self) -> Box<log::Log + 'static>;

    fn print(&self, m: &str) -> Result<()>;

    fn print_error(&self, m: &str, p: &core::ErrorPos) -> Result<()>;

    fn print_root_error(&self, e: &Error) -> Result<()>;
}
