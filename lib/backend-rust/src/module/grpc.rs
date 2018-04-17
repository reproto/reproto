//! gRPC module for Rust.

use backend::Initializer;
use Options;

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;
}
