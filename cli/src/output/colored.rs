use super::{LockableWrite, Output};
use ansi_term::Colour::{self, Blue, Red};
use log;
use reproto_core::errors::Result;
use reproto_core::{Source, Span};
use std::io;

pub struct Colored<T> {
    out: T,
}

impl<T> Colored<T>
where
    T: LockableWrite,
{
    pub fn new(out: T) -> Colored<T> {
        Colored { out }
    }

    fn print_positional(&self, source: &Source, span: &Span, m: &str, color: Colour) -> Result<()> {
        use std::cmp::max;
        use std::iter::repeat;

        let mut o = self.out.lock();

        let (line_str, line, (s, e)) =
            reproto_core::utils::find_line(source.read()?, (span.start, span.end))?;

        let line_no = format!("{:>3}:", line + 1);

        let mut indicator = String::new();

        indicator.extend(repeat(' ').take(line_no.len() + s + 1));
        indicator.extend(repeat('^').take(max(1, e - s)));

        writeln!(o, "{}:{}:{}-{}:", source, line + 1, s + 1, e + 1)?;
        writeln!(o, "{} {}", Blue.paint(line_no), line_str)?;
        writeln!(
            o,
            "{}{}{}",
            color.paint(indicator),
            color.paint(" - "),
            color.paint(m)
        )?;

        Ok(())
    }
}

impl<T> Output for Colored<T>
where
    T: LockableWrite,
{
    fn lock<'a>(&'a self) -> Box<dyn io::Write + 'a> {
        self.out.lock()
    }

    fn error_message(&self, m: &str) -> Result<String> {
        Ok(format!("{}", Red.paint(m)))
    }

    fn logger(&self) -> Box<dyn log::Log> {
        Box::new(ColoredLogger {
            out: self.out.open_new(),
        })
    }

    fn print(&self, m: &str) -> Result<()> {
        let mut o = self.out.lock();
        writeln!(o, "{}", Red.paint(m))?;
        Ok(())
    }

    fn print_info(&self, source: &Source, span: &Span, m: &str) -> Result<()> {
        self.print_positional(source, span, m, Colour::Yellow)
    }

    fn print_error(&self, source: &Source, span: &Span, m: &str) -> Result<()> {
        self.print_positional(source, span, m, Colour::Red)
    }
}

pub struct ColoredLogger<T> {
    out: T,
}

impl<T> log::Log for ColoredLogger<T>
where
    T: LockableWrite,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        use log::Level::*;

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

    fn flush(&self) {}
}
