//! Options for java code generation.

use crate::codegen::Generators;

pub struct Options {
    /// Build setters?
    pub build_setters: bool,
    /// Build getters?
    pub build_getters: bool,
    /// Build a constructor?
    pub build_constructor: bool,
    /// Build a Object#GetHashCode() implementation.
    pub build_hash_code: bool,
    /// Build a Object#Equals() implementation.
    pub build_equals: bool,
    /// Build a Object#ToString() implementation.
    pub build_to_string: bool,
    /// Do not generate methods in service interface.
    pub suppress_service_methods: bool,
    /// Access to registered generators.
    pub(crate) gen: Generators,
}

impl Options {
    pub fn new() -> Self {
        Self {
            build_setters: true,
            build_getters: true,
            build_constructor: true,
            build_hash_code: true,
            build_equals: true,
            build_to_string: true,
            suppress_service_methods: false,
            gen: Generators::default(),
        }
    }
}
