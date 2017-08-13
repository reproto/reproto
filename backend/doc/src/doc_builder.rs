use super::*;
use codeviz_common::{ElementFormat, ElementFormatter};
use std::fmt::{self, Write};

pub trait DocBuilder {
    fn write_str(&mut self, string: &str) -> fmt::Result;

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;

    fn new_line(&mut self) -> Result<()>;

    fn new_line_unless_empty(&mut self) -> Result<()>;

    fn indent(&mut self);

    fn unindent(&mut self);
}

pub struct DefaultDocBuilder<'a, W>
where
    W: fmt::Write + 'a,
{
    formatter: ElementFormatter<'a, W>,
}

impl<'a, W> DefaultDocBuilder<'a, W>
where
    W: fmt::Write,
{
    pub fn new(write: &'a mut W) -> DefaultDocBuilder<'a, W> {
        DefaultDocBuilder { formatter: ElementFormatter::new(write) }
    }
}

impl<'a, W> DocBuilder for DefaultDocBuilder<'a, W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.formatter.write_str(string)
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.formatter.write_fmt(args)
    }

    fn new_line(&mut self) -> Result<()> {
        self.formatter.new_line()?;
        Ok(())
    }

    fn new_line_unless_empty(&mut self) -> Result<()> {
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
