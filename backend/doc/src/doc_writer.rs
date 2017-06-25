use std::fmt::{self, Write};

pub struct DocWriter<'a> {
    dest: &'a mut Vec<String>,
    buffer: String,
}

impl<'a> DocWriter<'a> {
    pub fn new(dest: &'a mut Vec<String>) -> DocWriter<'a> {
        DocWriter {
            dest: dest,
            buffer: String::new(),
        }
    }
}

impl<'a> Write for DocWriter<'a> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        Write::write_str(&mut self.buffer, string)
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        Write::write_fmt(&mut self.buffer, args)
    }
}

/// Push the buffer onto the destination when writer is dropped.
impl<'a> Drop for DocWriter<'a> {
    fn drop(&mut self) {
        self.dest.push(self.buffer.clone());
    }
}
