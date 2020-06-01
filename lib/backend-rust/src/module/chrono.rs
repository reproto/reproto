//! Chrono module for Rust.

use crate::backend::Initializer;
use crate::core::errors::*;
use crate::Options;
use genco::rust::imported;

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
