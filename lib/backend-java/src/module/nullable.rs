use listeners::{Configure, Listeners};

pub struct Module;

impl Listeners for Module {
    fn configure(&self, e: Configure) {
        e.options.nullable = true;
    }
}
