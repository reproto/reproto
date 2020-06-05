//! Options for java code generation.

use crate::codegen;

pub struct Options {
    /// Should fields be nullable?
    pub(crate) nullable: bool,
    /// Should the type be immutable?
    pub(crate) immutable: bool,
    /// Build setters?
    pub(crate) build_setters: bool,
    /// Build getters?
    pub(crate) build_getters: bool,
    /// Build a constructor?
    pub(crate) build_constructor: bool,
    /// Build a Object#hashCode() implementation.
    pub(crate) build_hash_code: bool,
    /// Build a Object#equals() implementation.
    pub(crate) build_equals: bool,
    /// Build a Object#toString() implementation.
    pub(crate) build_to_string: bool,
    /// Generators used.
    pub(crate) gen: codegen::Generators,
}

impl Options {
    pub fn new() -> Self {
        Self {
            nullable: false,
            immutable: true,
            build_setters: true,
            build_getters: true,
            build_constructor: true,
            build_hash_code: true,
            build_equals: true,
            build_to_string: true,
            gen: codegen::Generators::default(),
        }
    }
}
