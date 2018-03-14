//! gRPC module for Rust.

use Options;
use backend::Initializer;
use core::errors::Result;

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        options.codable = true;
        Ok(())
    }
}
