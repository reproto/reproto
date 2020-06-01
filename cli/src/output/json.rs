use super::{LockableWrite, Output};
use crate::core::errors::*;
use crate::core::flavored::RpName;
use crate::core::{Encoding, Source, Span, SymbolKind};
use log;
use serde_json;
use std::io;
use std::path::PathBuf;

const NL: u8 = '\n' as u8;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "log")]
    Log { level: String, message: String },
    #[serde(rename = "diagnostics")]
    Diagnostics {
        message: String,
        path: PathBuf,
        range: Range,
    },
    #[serde(rename = "symbol")]
    Symbol {
        kind: SymbolKind,
        name: String,
        package: String,
        path: PathBuf,
        range: Range,
    },
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Serialize)]
pub struct Range {
    line_start: usize,
    col_start: usize,
    line_end: usize,
    col_end: usize,
}

pub struct Json<T> {
    out: T,
}

pub struct JsonLogger<T> {
    out: T,
}

impl<T> Json<T>
where
    T: LockableWrite,
{
    pub fn new(out: T) -> Json<T> {
        Json { out: out }
    }

    fn print_diagnostics(&self, source: &Source, span: &Span, m: &str) -> Result<()> {
        if let Some(path) = source.path() {
            let (start, end) = source.span_to_range(*span, Encoding::Utf8)?;

            let m = Message::Diagnostics {
                message: m.to_string(),
                path: path.to_owned(),
                range: Range {
                    line_start: start.line,
                    col_start: start.col,
                    line_end: end.line,
                    col_end: end.col,
                },
            };

            let mut out = self.out.lock();
            serde_json::to_writer(&mut out, &m)?;
            out.write(&[NL])?;
        }

        Ok(())
    }
}

impl<T> log::Log for JsonLogger<T>
where
    T: LockableWrite,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let m = Message::Log {
                level: record.level().to_string(),
                message: record.args().to_string(),
            };

            let mut out = self.out.lock();
            serde_json::to_writer(&mut out, &m).expect("failed to serialize");
            out.write(&[NL]).expect("failed to serializer");
        }
    }

    fn flush(&self) {}
}

impl<T> Output for Json<T>
where
    T: 'static + LockableWrite,
{
    fn lock<'a>(&'a self) -> Box<dyn io::Write + 'a> {
        self.out.lock()
    }

    fn logger(&self) -> Box<dyn log::Log + 'static> {
        Box::new(JsonLogger {
            out: self.out.open_new(),
        })
    }

    fn print(&self, m: &str) -> Result<()> {
        let m = Message::Error {
            message: m.to_string(),
        };

        let mut out = self.out.lock();
        serde_json::to_writer(&mut out, &m)?;
        out.write(&[NL])?;
        Ok(())
    }

    fn print_info(&self, source: &Source, p: &Span, m: &str) -> Result<()> {
        self.print_diagnostics(source, p, m)
    }

    fn print_error(&self, source: &Source, p: &Span, m: &str) -> Result<()> {
        self.print_diagnostics(source, p, m)
    }

    fn print_symbol(
        &self,
        source: &Source,
        kind: SymbolKind,
        span: &Span,
        name: &RpName,
    ) -> Result<()> {
        let path = match source.path() {
            Some(path) => path,
            None => return Ok(()),
        };

        let path = if !path.is_absolute() {
            path.canonicalize()?
        } else {
            path.to_owned()
        };

        let (start, end) = source.span_to_range(*span, Encoding::Utf8)?;

        let m = Message::Symbol {
            kind,
            name: name.path.join("::"),
            package: name.package.to_string(),
            path: path.to_owned(),
            range: Range {
                line_start: start.line,
                col_start: start.col,
                line_end: end.line,
                col_end: end.col,
            },
        };

        let mut out = self.out.lock();
        serde_json::to_writer(&mut out, &m)?;
        out.write(&[NL])?;

        Ok(())
    }
}
