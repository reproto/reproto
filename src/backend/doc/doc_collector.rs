//! # Collector of results from the doc backend

use std::fmt::Write;
use super::*;

pub struct DocCollector {
    buffer: String,
}

impl<'a> Collecting<'a> for DocCollector {
    type Processor = DocCompiler<'a>;

    fn new() -> Self {
        DocCollector { buffer: String::new() }
    }

    fn into_bytes(self, compiler: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();

        compiler.processor
            .write_doc(&mut out, move |out| {
                out.write_str(&self.buffer)?;
                Ok(())
            })?;

        Ok(out.into_bytes())
    }
}

impl Write for DocCollector {
    fn write_str(&mut self, other: &str) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.buffer.write_str(other)
    }
}
