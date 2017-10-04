/// Module that adds lombok annotations to generated classes.
use super::*;
use genco::{Cons, Java};
use genco::java::{Class, imported};

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

    fn class_added<'a>(&self, _names: &[Cons<'a>], spec: &mut Class<'a>) -> Result<()> {
        spec.annotation(toks!["@", self.data.clone()]);
        Ok(())
    }
}
