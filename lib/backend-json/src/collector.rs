use backend::IntoBytes;
use core::errors::*;
use json_compiler::JsonCompiler;
use std::fmt::Write;

pub struct Collector {
    buffer: String,
}

impl<'a> Default for Collector {
    fn default() -> Self {
        Collector {
            buffer: String::new(),
        }
    }
}

impl<'a> IntoBytes<JsonCompiler<'a>> for Collector {
    fn into_bytes(self, _: &JsonCompiler<'a>) -> Result<Vec<u8>> {
        Ok(self.buffer.into_bytes())
    }
}

impl Write for Collector {
    fn write_str(&mut self, other: &str) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.buffer.write_str(other)
    }
}
