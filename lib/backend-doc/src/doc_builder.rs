use std::fmt;

pub struct DocBuilder<'a> {
    empty: bool,
    write: &'a mut dyn fmt::Write,
}

impl<'a> DocBuilder<'a> {
    pub fn new(write: &'a mut dyn fmt::Write) -> DocBuilder<'a> {
        DocBuilder { empty: true, write }
    }
}

impl<'a> DocBuilder<'a> {
    pub fn write_str(&mut self, string: &str) -> fmt::Result {
        self.empty = false;
        self.write.write_str(string)
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.empty = false;
        self.write.write_fmt(args)
    }

    pub fn new_line(&mut self) -> fmt::Result {
        self.write.write_str("\n")?;
        self.empty = true;
        Ok(())
    }

    pub fn new_line_unless_empty(&mut self) -> fmt::Result {
        if !self.empty {
            self.write.write_str("\n")?;
            self.empty = true;
        }

        Ok(())
    }

    pub fn indent(&mut self) {}

    pub fn unindent(&mut self) {}
}
