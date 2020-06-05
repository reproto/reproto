mod colored;
mod json;
mod non_colored;

pub use self::colored::Colored;
pub use self::json::Json;
pub use self::non_colored::NonColored;
use core::errors::{Error, Result};
use core::flavored::RpName;
use core::{self, Diagnostic, Reported, Source};
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

    fn lock<'a>(&'a self) -> Box<dyn Write + 'a>;
}

impl LockableWrite for io::Stdout {
    fn open_new(&self) -> Self {
        io::stdout()
    }

    fn lock<'a>(&'a self) -> Box<dyn Write + 'a> {
        Box::new(self.lock())
    }
}

pub trait Output {
    fn lock<'a>(&'a self) -> Box<dyn Write + 'a>;

    fn handle_context(&self, diagnostics: &[Reported]) -> Result<()> {
        for d in diagnostics {
            match *d {
                Reported::Diagnostics(ref diagnostics) => {
                    for d in diagnostics.items() {
                        self.report_diagnostic(&diagnostics.source, d)?;
                    }
                }
                Reported::SourceDiagnostics(ref diagnostics) => {
                    for d in diagnostics.items() {
                        self.report_diagnostic(&d.0, &d.1)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn report_diagnostic(&self, source: &Source, item: &Diagnostic) -> Result<()> {
        match *item {
            Diagnostic::Info {
                ref span,
                ref message,
            } => {
                self.print_info(source, span, message.as_str())?;
            }
            Diagnostic::Error {
                ref span,
                ref message,
            } => {
                self.print_error(source, span, message.as_str())?;
            }
            Diagnostic::Symbol {
                ref kind,
                ref span,
                ref name,
            } => {
                self.print_symbol(source, *kind, span, name)?;
            }
        }

        Ok(())
    }

    /// Handle any errors.
    fn handle_error(&self, e: &Error, reason: Option<&str>) -> Result<()> {
        for e in e.causes() {
            self.error(e, reason)?;

            for e in e.suppressed() {
                self.handle_error(e, Some("suppressed"))?;
            }

            if let Some(backtrace) = e.backtrace() {
                self.print(&format!("{:?}", backtrace))?;
            }
        }

        Ok(())
    }

    fn error(&self, e: &Error, reason: Option<&str>) -> Result<()> {
        let message = match reason {
            Some(reason) => format!("{}: {}", reason, e.message()),
            None => e.message().to_string(),
        };

        self.print(&message)?;

        for e in e.causes().skip(1) {
            let msg = self.error_message(format!("  caused by: {}", e.message()).as_str())?;
            self.print(msg.as_str())?;
        }

        Ok(())
    }

    fn error_message(&self, m: &str) -> Result<String> {
        Ok(m.to_string())
    }

    fn logger(&self) -> Box<dyn log::Log + 'static>;

    fn print(&self, m: &str) -> Result<()>;

    fn print_info(&self, source: &core::Source, p: &core::Span, m: &str) -> Result<()>;

    fn print_error(&self, source: &core::Source, p: &core::Span, m: &str) -> Result<()>;

    fn print_symbol(
        &self,
        _source: &core::Source,
        _kind: core::SymbolKind,
        _pos: &core::Span,
        _name: &RpName,
    ) -> Result<()> {
        Ok(())
    }
}
