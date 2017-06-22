use std::fmt::{Arguments, Result, Write};

pub trait DocBuilder {
    fn write_str(&mut self, string: &str) -> Result;

    fn write_fmt(&mut self, args: Arguments) -> Result;
}

impl DocBuilder for String {
    fn write_str(&mut self, string: &str) -> Result {
        Write::write_str(self, string)
    }

    fn write_fmt(&mut self, args: Arguments) -> Result {
        Write::write_fmt(self, args)
    }
}
