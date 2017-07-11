use errors::*;
use log;
use reproto_core::ErrorPos;
use super::{LockableWrite, Output, find_line};

pub struct NonColored {
    out: Box<LockableWrite>,
}

pub struct NonColoredLogger {
    out: Box<LockableWrite>,
}

impl NonColored {
    pub fn new(out: Box<LockableWrite>) -> NonColored {
        NonColored { out: out }
    }
}

impl log::Log for NonColoredLogger {
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


impl Output for NonColored {
    fn logger(&self) -> Box<log::Log> {
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

        let (line_str, line, (s, e)) = find_line(&p.path, (p.start, p.end))?;

        let line_no = format!("{:>3}:", line + 1);

        let mut indicator = String::new();

        indicator.extend(repeat(' ').take(line_no.len() + s + 1));
        indicator.extend(repeat('^').take(max(1, e - s)));

        writeln!(o, "{}:{}:{}-{}:", p.path.display(), line + 1, s + 1, e + 1)?;
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
