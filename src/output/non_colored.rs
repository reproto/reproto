use core::ErrorPos;
use errors::*;
use log;
use super::{LockableWrite, Output, find_line};

pub struct NonColored<T> {
    out: T,
}

pub struct NonColoredLogger<T> {
    out: T,
}

impl<T> NonColored<T> {
    pub fn new(out: T) -> NonColored<T> {
        NonColored { out: out }
    }
}

impl<T> log::Log for NonColoredLogger<T>
    where T: LockableWrite
{
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Debug
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            let mut out = self.out.lock();
            writeln!(out, "{}: {}", record.level(), record.args()).unwrap();
        }
    }
}


impl<T> Output for NonColored<T>
    where T: 'static + LockableWrite
{
    fn logger(&self) -> Box<log::Log + 'static> {
        Box::new(NonColoredLogger { out: self.out.open_new() })
    }

    fn print(&self, m: &str) -> Result<()> {
        let mut o = self.out.lock();
        writeln!(o, "ERROR: {}", m)?;
        Ok(())
    }

    fn print_error(&self, m: &str, p: &ErrorPos) -> Result<()> {
        use std::iter::repeat;
        use std::cmp::max;

        let mut o = self.out.lock();
        let object = p.object.lock().map_err(|_| ErrorKind::PoisonError)?;

        let (line_str, line, (s, e)) = find_line(object.read()?, (p.start, p.end))?;

        let line_no = format!("{:>3}:", line + 1);

        let mut indicator = String::new();

        indicator.extend(repeat(' ').take(line_no.len() + s + 1));
        indicator.extend(repeat('^').take(max(1, e - s)));

        writeln!(o, "{}:{}:{}-{}:", object, line + 1, s + 1, e + 1)?;
        writeln!(o, "{} {}", line_no, line_str)?;
        writeln!(o, "{}{}{}", indicator, " - ", m)?;

        Ok(())
    }

    fn print_root_error(&self, e: &Error) -> Result<()> {
        let mut o = self.out.lock();

        writeln!(o, "{}", e)?;

        for cause in e.iter().skip(1) {
            writeln!(o, "  caused by: {}", cause)?;
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(o, "backtrace: {:?}", backtrace)?;
        }

        Ok(())
    }
}
