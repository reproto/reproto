mod colored;
mod non_colored;

pub use self::colored::Colored;
pub use self::non_colored::NonColored;
use backend;
use core::{self, Context, ContextItem};
use errors::*;
use log;
use parser;
use repository;
use semck::Violation;
use std::fmt;
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
            // already being reported through the context.
            Context => true,
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

    fn handle_context(&self, ctx: &Context) -> Result<()> {
        let errors = ctx.errors()?;

        for e in errors.iter() {
            match *e {
                ContextItem::ErrorPos(ref pos, ref message) => {
                    self.print_error(message.as_str(), pos)?;
                }
                ContextItem::InfoPos(ref pos, ref message) => {
                    self.print_info(message.as_str(), pos)?;
                }
            }
        }

        Ok(())
    }

    /// Handle any errors.
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
            SemckViolation(index, ref violation) => {
                self.semck_violation(index, violation)?;
                true
            }
            _ => false,
        };

        Ok(out)
    }

    fn semck_violation(&self, _index: usize, violation: &Violation) -> Result<()> {
        use self::Violation::*;

        match *violation {
            DeclRemoved(ref c, ref reg) => {
                self.print_error(
                    format!("{}: declaration removed", c.describe()).as_str(),
                    reg,
                )?;
            }
            DeclAdded(ref c, ref reg) => {
                self.print_error(format!("{}: declaration added", c.describe()).as_str(), reg)?;
            }
            RemoveField(ref c, ref field) => {
                self.print_error(format!("{}: field removed", c.describe()).as_str(), field)?;
            }
            RemoveVariant(ref c, ref field) => {
                self.print_error(format!("{}: variant removed", c.describe()).as_str(), field)?;
            }
            AddField(ref c, ref field) => {
                self.print_error(format!("{}: field added", c.describe()).as_str(), field)?;
            }
            AddVariant(ref c, ref field) => {
                self.print_error(format!("{}: variant added", c.describe()).as_str(), field)?;
            }
            FieldTypeChange(ref c, ref from_type, ref from, ref to_type, ref to) => {
                self.print_error(
                    format!("{}: type changed to `{}`", c.describe(), to_type).as_str(),
                    to,
                )?;

                self.print_error(format!("from `{}`", from_type).as_str(), from)?;
            }
            FieldNameChange(ref c, ref from_name, ref from, ref to_name, ref to) => {
                self.print_error(
                    format!("{}: name changed to `{}`", c.describe(), to_name).as_str(),
                    to,
                )?;

                self.print_error(format!("from `{}`", from_name).as_str(), from)?;
            }
            VariantOrdinalChange(ref c, ref from_ordinal, ref from, ref to_ordinal, ref to) => {
                self.print_error(
                    format!("{}: ordinal changed to `{}`", c.describe(), to_ordinal).as_str(),
                    to,
                )?;

                self.print_error(format!("from `{}`", from_ordinal).as_str(), from)?;
            }
            FieldRequiredChange(ref c, ref from, ref to) => {
                self.print_error(
                    format!("{}: field changed to be required`", c.describe(),).as_str(),
                    to,
                )?;

                self.print_error("from here", from)?;
            }
            AddRequiredField(ref c, ref field) => {
                self.print_error(
                    format!("{}: required field added", c.describe(),).as_str(),
                    field,
                )?;
            }
            FieldModifierChange(ref c, ref from, ref to) => {
                self.print_error(
                    format!("{}: field modifier changed", c.describe(),).as_str(),
                    to,
                )?;

                self.print_error("from here", from)?;
            }
            AddEndpoint(ref c, ref pos) => {
                self.print_error(format!("{}: endpoint added", c.describe()).as_str(), pos)?;
            }
            RemoveEndpoint(ref c, ref pos) => {
                self.print_error(format!("{}: endpoint removed", c.describe()).as_str(), pos)?;
            }
            EndpointRequestChange(ref c, ref from_channel, ref from, ref to_channel, ref to) => {
                self.print_error(
                    format!(
                        "{}: request type changed to `{}`",
                        c.describe(),
                        PrintChannelInfo(to_channel)
                    ).as_str(),
                    to,
                )?;

                self.print_error(
                    format!("from `{}`", PrintChannelInfo(from_channel)).as_str(),
                    from,
                )?;
            }
            EndpointResponseChange(ref c, ref from_channel, ref from, ref to_channel, ref to) => {
                self.print_error(
                    format!(
                        "{}: response type changed to `{}`",
                        c.describe(),
                        PrintChannelInfo(to_channel)
                    ).as_str(),
                    to,
                )?;

                self.print_error(
                    format!("from `{}`", PrintChannelInfo(from_channel)).as_str(),
                    from,
                )?;
            }
        }

        Ok(())
    }

    fn logger(&self) -> Box<log::Log + 'static>;

    fn print(&self, m: &str) -> Result<()>;

    fn print_info(&self, m: &str, p: &core::ErrorPos) -> Result<()>;

    fn print_error(&self, m: &str, p: &core::ErrorPos) -> Result<()>;

    fn print_root_error(&self, e: &Error) -> Result<()>;
}

/// Helper struct to display information on channels.
struct PrintChannelInfo<'a>(&'a Option<core::RpChannel>);

impl<'a> fmt::Display for PrintChannelInfo<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            None => write!(fmt, "*empty*"),
            Some(ref channel) => write!(fmt, "{}", channel),
        }
    }
}
