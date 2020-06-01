//! Options for java code generation.

use crate::codegen::{
    ClassCodegen, Codegen, EnumCodegen, InterfaceCodegen, ServiceCodegen, TupleCodegen,
    TypeFieldCodegen,
};

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
    /// Hook to generate code called in the root of the declarations.
    pub root_generators: Vec<Box<dyn Codegen>>,
    /// Hook to run class generators.
    pub class_generators: Vec<Box<dyn ClassCodegen>>,
    /// Hook to run service generators.
    pub service_generators: Vec<Box<dyn ServiceCodegen>>,
    /// Hook to run tuple generators.
    pub tuple_generators: Vec<Box<dyn TupleCodegen>>,
    /// Hook to run interface generators.
    pub interface_generators: Vec<Box<dyn InterfaceCodegen>>,
    /// Hook to run enum generators.
    pub enum_generators: Vec<Box<dyn EnumCodegen>>,
    /// Hook to run type-field generators.
    pub type_field_generators: Vec<Box<dyn TypeFieldCodegen>>,
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
            root_generators: Vec::new(),
            class_generators: Vec::new(),
            service_generators: Vec::new(),
            tuple_generators: Vec::new(),
            interface_generators: Vec::new(),
            enum_generators: Vec::new(),
            type_field_generators: Vec::new(),
        }
    }
}
