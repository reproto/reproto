mod colored;
mod json;
mod non_colored;

pub use self::colored::Colored;
pub use self::json::Json;
pub use self::non_colored::NonColored;
use core::errors::*;
use core::flavored::RpName;
use core::{self, ContextItem};
use log;
use std::io::{self, Write};

/// Output format to print stuff using.
pub enum OutputFormat {
    /// All output must be printed as JSON, one message per line.
    Json,
    /// All output must be printed in a human-readable format.
    Human,
}

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

pub trait Output {
    fn lock<'a>(&'a self) -> Box<Write + 'a>;

    fn handle_context(&self, errors: &[ContextItem]) -> Result<()> {
        for e in errors.iter() {
            match *e {
                ContextItem::InfoPos(ref pos, ref message) => {
                    self.print_info(message.as_str(), pos)?;
                }
                ContextItem::ErrorPos(ref pos, ref message) => {
                    self.print_error(message.as_str(), pos)?;
                }
                ContextItem::Symbol {
                    ref kind,
                    ref pos,
                    ref name,
                } => {
                    self.print_symbol(*kind, pos, name)?;
                }
            }
        }

        Ok(())
    }

    /// Handle any errors.
    fn handle_error(&self, e: &Error) -> Result<()> {
        for e in e.causes() {
            if let Some(pos) = e.pos() {
                self.print_error(e.message(), pos)?;
            } else {
                self.error(e)?;
            }

            for e in e.suppressed() {
                self.handle_error(e)?;
            }

            if let Some(backtrace) = e.backtrace() {
                self.print(&format!("{:?}", backtrace))?;
            }
        }

        Ok(())
    }

    fn error(&self, e: &Error) -> Result<()> {
        if let Some(p) = e.pos() {
            self.print_error(e.message(), p)?;
        } else {
            self.print(&e.message())?;
        }

        for e in e.causes().skip(1) {
            let msg = self.error_message(format!("  caused by: {}", e.message()).as_str())?;

            if let Some(p) = e.pos() {
                self.print_error(msg.as_str(), p)?;
            } else {
                self.print(msg.as_str())?;
            }
        }

        Ok(())
    }

    fn error_message(&self, m: &str) -> Result<String> {
        Ok(m.to_string())
    }

    fn logger(&self) -> Box<log::Log + 'static>;

    fn print(&self, m: &str) -> Result<()>;

    fn print_info(&self, m: &str, p: &core::ErrorPos) -> Result<()>;

    fn print_error(&self, m: &str, p: &core::ErrorPos) -> Result<()>;

    fn print_symbol(
        &self,
        _kind: core::SymbolKind,
        _pos: &core::ErrorPos,
        _name: &RpName,
    ) -> Result<()> {
        Ok(())
    }
}
