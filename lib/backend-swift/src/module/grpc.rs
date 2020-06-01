//! gRPC module for Rust.

use crate::backend::Initializer;
use crate::Options;

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;
}
