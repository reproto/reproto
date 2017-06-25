pub struct JsOptions {
    pub build_getters: bool,
    pub build_constructor: bool,
}

impl JsOptions {
    pub fn new() -> JsOptions {
        JsOptions {
            build_getters: false,
            build_constructor: true,
        }
    }
}
