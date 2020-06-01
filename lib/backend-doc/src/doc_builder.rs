use genco::Formatter;
use std::fmt::{self, Write};

pub struct DocBuilder<'a> {
    formatter: Formatter<'a>,
}

impl<'a> DocBuilder<'a> {
    pub fn new(write: &'a mut dyn fmt::Write) -> DocBuilder<'a> {
        DocBuilder {
            formatter: Formatter::new(write),
        }
    }
}

impl<'a> DocBuilder<'a> {
    pub fn write_str(&mut self, string: &str) -> fmt::Result {
        self.formatter.write_str(string)
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.formatter.write_fmt(args)
    }

    pub fn new_line(&mut self) -> fmt::Result {
        self.formatter.new_line()?;
        Ok(())
    }

    pub fn new_line_unless_empty(&mut self) -> fmt::Result {
        self.formatter.new_line_unless_empty()?;
        Ok(())
    }

    pub fn indent(&mut self) {
        self.formatter.indent();
    }

    pub fn unindent(&mut self) {
        self.formatter.unindent();
    }
}
