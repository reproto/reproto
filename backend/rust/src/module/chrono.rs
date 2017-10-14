//! Chrono module for Rust.

use backend::errors::*;
use genco::Rust;
use genco::rust::imported_ref;
use listeners::Listeners;
use rust_options::RustOptions;

pub struct Module {
    datetime: Rust<'static>,
    offset_utc: Rust<'static>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            datetime: imported_ref("chrono", "DateTime"),
            offset_utc: imported_ref("chrono::offset", "Utc"),
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
