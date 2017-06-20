/// Module that adds lombok annotations to generated classes.
use super::*;

pub struct Module {
    data: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module { data: Type::class("lombok", "Data") }
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

    fn class_added(&self, event: &mut ClassAdded) -> Result<()> {
        event.spec.push_annotation(&self.data);
        Ok(())
    }
}
