/// Module that adds lombok annotations to generated classes.
use super::processor;

use codegen::java::*;
use errors::*;

pub struct Module {
    data: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module { data: Type::class("lombok", "Data") }
    }
}

impl processor::Listeners for Module {
    fn configure(&self, options: &mut processor::ProcessorOptions) -> Result<()> {
        // lombok builds getters
        options.build_getters = false;
        // lombok builds constructor
        options.build_constructor = false;
        Ok(())
    }

    fn class_added(&self, _fields: &Vec<processor::Field>, class: &mut ClassSpec) -> Result<()> {
        class.push_annotation(&self.data);
        Ok(())
    }
}
