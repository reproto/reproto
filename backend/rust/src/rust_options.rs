use genco::{Rust, Tokens};

pub struct RustOptions {
    pub datetime: Option<Tokens<'static, Rust<'static>>>,
}

impl RustOptions {
    pub fn new() -> RustOptions {
        RustOptions { datetime: None }
    }
}
