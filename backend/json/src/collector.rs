use std::fmt::Write;
use super::*;

pub struct Collector {
    buffer: String,
}

impl<'a> Collecting<'a> for Collector {
    type Processor = JsonCompiler<'a>;

    fn new() -> Self {
        Collector { buffer: String::new() }
    }

    fn into_bytes(self, _: &Self::Processor) -> Result<Vec<u8>> {
        Ok(self.buffer.into_bytes())
    }
}

impl Write for Collector {
    fn write_str(&mut self, other: &str) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.buffer.write_str(other)
    }
}
