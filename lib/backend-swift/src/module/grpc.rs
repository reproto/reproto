//! gRPC module for Rust.

use crate::Options;
use backend::Initializer;

pub(crate) struct Module {}

impl Module {
    pub(crate) fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;
}
