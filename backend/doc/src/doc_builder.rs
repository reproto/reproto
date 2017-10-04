use genco::Formatter;
use std::fmt::{self, Write};

pub trait DocBuilder {
    fn write_str(&mut self, string: &str) -> fmt::Result;

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;

    fn new_line(&mut self) -> fmt::Result;

    fn new_line_unless_empty(&mut self) -> fmt::Result;

    fn indent(&mut self);

    fn unindent(&mut self);
}

pub struct DefaultDocBuilder<'a> {
    formatter: Formatter<'a>,
}

impl<'a> DefaultDocBuilder<'a> {
    pub fn new(write: &'a mut fmt::Write) -> DefaultDocBuilder<'a> {
        DefaultDocBuilder { formatter: Formatter::new(write) }
    }
}

impl<'a> DocBuilder for DefaultDocBuilder<'a> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.formatter.write_str(string)
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.formatter.write_fmt(args)
    }

    fn new_line(&mut self) -> fmt::Result {
        self.formatter.new_line()?;
        Ok(())
    }

    fn new_line_unless_empty(&mut self) -> fmt::Result {
        self.formatter.new_line_unless_empty()?;
        Ok(())
    }

    fn indent(&mut self) {
        self.formatter.indent();
    }

    fn unindent(&mut self) {
        self.formatter.unindent();
    }
}
