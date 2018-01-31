/// Plugin infrastructure for Rust Backend.
use super::rust_options::RustOptions;
use backend::errors::*;

pub trait Listeners {
    listeners_vec_default!(configure, RustOptions);
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    listeners_vec!(configure, RustOptions);
}
