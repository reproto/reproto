//! Chrono module for Rust.

use crate::flavored::Type;
use crate::Options;
use core::errors::Result;
use genco::lang::rust;

pub(crate) fn initialize(options: &mut Options) -> Result<()> {
    options.datetime = Some(Type::generic(
        rust::import("chrono", "DateTime"),
        rust::import("chrono::offset", "Utc"),
    ));
    Ok(())
}
