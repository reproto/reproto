//! Chrono module for Rust.

use core::errors::*;
use genco::Rust;
use genco::rust::imported;
use listeners::Listeners;
use rust_options::RustOptions;

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

impl Listeners for Module {
    fn configure(&self, options: &mut RustOptions) -> Result<()> {
        options.datetime = Some(toks![
            self.datetime.clone(),
            "<",
            self.offset_utc.clone(),
            ">",
        ]);

        Ok(())
    }
}
