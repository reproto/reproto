use super::{LockableWrite, Output};
use core::errors::*;
use core::{self, ErrorPos};
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

    fn print_positional(&self, m: &str, p: &ErrorPos) -> Result<()> {
        if let Some(path) = p.object.path() {
            let (line_start, line_end, col_start, col_end) =
                core::utils::find_range(p.object.read()?, (p.start, p.end))?;

            let m = Message::Diagnostics {
                message: m.to_string(),
                path: path.to_owned(),
                range: Range {
                    line_start,
                    col_start,
                    line_end,
                    col_end,
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
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Debug
    }

    fn log(&self, record: &log::LogRecord) {
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
}

impl<T> Output for Json<T>
where
    T: 'static + LockableWrite,
{
    fn lock<'a>(&'a self) -> Box<io::Write + 'a> {
        self.out.lock()
    }

    fn logger(&self) -> Box<log::Log + 'static> {
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

    fn print_info(&self, m: &str, p: &ErrorPos) -> Result<()> {
        self.print_positional(m, p)
    }

    fn print_error(&self, m: &str, p: &ErrorPos) -> Result<()> {
        self.print_positional(m, p)
    }
}
