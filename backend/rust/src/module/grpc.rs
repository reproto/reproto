//! gRPC module for Rust.

use listeners::Listeners;

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Listeners for Module {}
