use codegen::ServiceCodegen;

pub struct Options {
    pub build_getters: bool,
    pub build_constructor: bool,
    pub service_generators: Vec<Box<ServiceCodegen>>,
}

impl Options {
    pub fn new() -> Options {
        Options {
            build_getters: true,
            build_constructor: true,
            service_generators: Vec::new(),
        }
    }
}
