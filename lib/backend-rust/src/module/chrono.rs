//! Chrono module for Rust.

use backend::Initializer;
use core::errors::*;
use genco::rust::imported;
use Options;

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        options.datetime = Some(
            imported("chrono", "DateTime").with_arguments(vec![imported("chrono::offset", "Utc")]),
        );

        Ok(())
    }
}
