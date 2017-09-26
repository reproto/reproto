use super::{LockableWrite, Output, find_line};
use ansi_term::Colour::{Blue, Red};
use core::ErrorPos;
use errors::*;
use log;

pub struct Colored<T> {
    out: T,
}

impl<T> Colored<T> {
    pub fn new(out: T) -> Colored<T> {
        Colored { out: out }
    }
}

impl<T> Output for Colored<T>
where
    T: 'static + LockableWrite,
{
    fn logger(&self) -> Box<log::Log + 'static> {
        Box::new(ColoredLogger { out: self.out.open_new() })
    }

    fn print(&self, m: &str) -> Result<()> {
        let mut o = self.out.lock();
        writeln!(o, "ERROR: {}", Red.paint(m.as_ref()))?;
        Ok(())
    }

    fn print_error(&self, m: &str, p: &ErrorPos) -> Result<()> {
        use std::iter::repeat;
        use std::cmp::max;

        let mut o = self.out.lock();

        let (line_str, line, (s, e)) = find_line(p.object.read()?, (p.start, p.end))?;

        let line_no = format!("{:>3}:", line + 1);

        let mut indicator = String::new();

        indicator.extend(repeat(' ').take(line_no.len() + s + 1));
        indicator.extend(repeat('^').take(max(1, e - s)));

        writeln!(o, "{}:{}:{}-{}:", p.object, line + 1, s + 1, e + 1)?;
        writeln!(o, "{} {}", Blue.paint(line_no), line_str)?;
        writeln!(
            o,
            "{}{}{}",
            Red.paint(indicator),
            Red.paint(" - "),
            Red.paint(m.as_ref())
        )?;

        Ok(())
    }

    fn print_root_error(&self, e: &Error) -> Result<()> {
        use ansi_term::Colour::Red;

        let mut o = self.out.lock();

        writeln!(o, "{}", Red.paint(format!("{}", e)))?;

        for cause in e.iter().skip(1) {
            writeln!(o, "  caused by: {}", cause)?;
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(o, "backtrace: {:?}", backtrace)?;
        }

        Ok(())
    }
}

pub struct ColoredLogger<T> {
    out: T,
}

impl<T> log::Log for ColoredLogger<T>
where
    T: LockableWrite,
{
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Debug
    }

    fn log(&self, record: &log::LogRecord) {
        use log::LogLevel::*;

        if self.enabled(record.metadata()) {
            let mut out = self.out.lock();

            let result = match record.level() {
                Error => writeln!(out, "ERROR - {}", Red.paint(format!("{}", record.args()))),
                Debug => writeln!(out, "DEBUG - {}", Blue.paint(format!("{}", record.args()))),
                level => writeln!(out, "{} - {}", level, record.args()),
            };

            result.unwrap();
        }
    }
}
