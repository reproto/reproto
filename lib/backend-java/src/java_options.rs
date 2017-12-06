/// Options for java code generation.

use genco::Java;
use codegen::Codegen;

pub struct JavaOptions {
    /// Should fields be nullable?
    pub nullable: bool,
    /// Should the type be immutable?
    pub immutable: bool,
    /// Build setters?
    pub build_setters: bool,
    /// Build getters?
    pub build_getters: bool,
    /// Build a constructor?
    pub build_constructor: bool,
    /// Build a Object#hashCode() implementation.
    pub build_hash_code: bool,
    /// Build a Object#equals() implementation.
    pub build_equals: bool,
    /// Build a Object#toString() implementation.
    pub build_to_string: bool,
    /// Container to use for asynchronous operations.
    pub async_container: Option<Java<'static>>,
    /// Do not generate methods in service interface.
    pub suppress_service_methods: bool,
    /// Hook to generate code called in the root of the declarations.
    pub root_generators: Vec<Box<Codegen>>,
}

impl JavaOptions {
    pub fn new() -> JavaOptions {
        JavaOptions {
            nullable: false,
            immutable: true,
            build_setters: true,
            build_getters: true,
            build_constructor: true,
            build_hash_code: true,
            build_equals: true,
            build_to_string: true,
            async_container: None,
            suppress_service_methods: false,
            root_generators: Vec::new(),
        }
    }
}
