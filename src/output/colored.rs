use ansi_term::Colour::{Blue, Red};
use errors::*;
use log;
use reproto_core::ErrorPos;
use super::{LockableWrite, Output, find_line};

pub struct Colored {
    out: Box<LockableWrite>,
}

impl Colored {
    pub fn new(out: Box<LockableWrite>) -> Colored {
        Colored { out: out }
    }
}

impl Output for Colored {
    fn logger(&self) -> Box<log::Log> {
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

        let (line_str, line, (s, e)) = find_line(&p.path, (p.start, p.end))?;

        let line_no = format!("{:>3}:", line + 1);

        let mut indicator = String::new();

        indicator.extend(repeat(' ').take(line_no.len() + s + 1));
        indicator.extend(repeat('^').take(max(1, e - s)));

        writeln!(o, "{}:{}:{}-{}:", p.path.display(), line + 1, s + 1, e + 1)?;
        writeln!(o, "{} {}", Blue.paint(line_no), line_str)?;
        writeln!(o,
                 "{}{}{}",
                 Red.paint(indicator),
                 Red.paint(" - "),
                 Red.paint(m.as_ref()))?;

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

pub struct ColoredLogger {
    out: Box<LockableWrite>,
}

impl log::Log for ColoredLogger {
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
