//! Chrono module for Rust.

use crate::flavored::Type;
use crate::Options;
use backend::Initializer;
use core::errors::Result;
use genco::lang::rust;

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        options.datetime = Some(Type::generic(
            rust::import("chrono", "DateTime"),
            rust::import("chrono::offset", "Utc"),
        ));
        Ok(())
    }
}
