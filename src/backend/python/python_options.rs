pub struct PythonOptions {
    pub build_getters: bool,
    pub build_constructor: bool,
}

impl PythonOptions {
    pub fn new() -> PythonOptions {
        PythonOptions {
            build_getters: true,
            build_constructor: true,
        }
    }
}
