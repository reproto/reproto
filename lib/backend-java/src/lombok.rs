//! Module that adds lombok annotations to generated classes.

use backend::errors::*;
use genco::Java;
use genco::java::imported;
use java_options::JavaOptions;
use listeners::{ClassAdded, Listeners};

pub struct Module {
    data: Java<'static>,
}

impl Module {
    pub fn new() -> Module {
        Module { data: imported("lombok", "Data") }
    }
}

impl Listeners for Module {
    fn configure(&self, options: &mut JavaOptions) -> Result<()> {
        // lombok builds these automatically.
        options.build_getters = false;
        options.build_constructor = false;
        options.build_hash_code = false;
        options.build_equals = false;
        options.build_to_string = false;
        Ok(())
    }

    fn class_added<'a>(&self, e: &mut ClassAdded) -> Result<()> {
        e.spec.annotation(toks!["@", self.data.clone()]);
        Ok(())
    }
}
