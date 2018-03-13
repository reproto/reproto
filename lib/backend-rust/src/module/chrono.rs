//! Chrono module for Rust.

use Options;
use backend::Initializer;
use core::errors::*;
use genco::Rust;
use genco::rust::imported;

pub struct Module {
    datetime: Rust<'static>,
    offset_utc: Rust<'static>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            datetime: imported("chrono", "DateTime"),
            offset_utc: imported("chrono::offset", "Utc"),
        }
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        options.datetime = Some(toks![
            self.datetime.clone(),
            "<",
            self.offset_utc.clone(),
            ">",
        ]);

        Ok(())
    }
}
