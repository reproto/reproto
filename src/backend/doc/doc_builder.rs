use codeviz::common::{ElementFormat, ElementFormatter};
use errors as e;
use std::fmt::{Arguments, Result, Write};

pub trait DocBuilder {
    fn write_str(&mut self, string: &str) -> Result;

    fn write_fmt(&mut self, args: Arguments) -> Result;

    fn new_line(&mut self) -> e::Result<()>;

    fn new_line_unless_empty(&mut self) -> e::Result<()>;

    fn indent(&mut self);

    fn unindent(&mut self);
}

pub struct DefaultDocBuilder<'a, W>
    where W: Write + 'a
{
    formatter: ElementFormatter<'a, W>,
}

impl<'a, W> DefaultDocBuilder<'a, W>
    where W: Write
{
    pub fn new(write: &'a mut W) -> DefaultDocBuilder<'a, W> {
        DefaultDocBuilder { formatter: ElementFormatter::new(write) }
    }
}

impl<'a, W> DocBuilder for DefaultDocBuilder<'a, W>
    where W: Write
{
    fn write_str(&mut self, string: &str) -> Result {
        self.formatter.write_str(string)
    }

    fn write_fmt(&mut self, args: Arguments) -> Result {
        self.formatter.write_fmt(args)
    }

    fn new_line(&mut self) -> e::Result<()> {
        self.formatter.new_line()?;
        Ok(())
    }

    fn new_line_unless_empty(&mut self) -> e::Result<()> {
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
