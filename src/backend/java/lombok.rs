/// Module that adds lombok annotations to generated classes.
use backend::*;
use codeviz::java::*;
use super::models as m;
use super::processor;

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
        // lombok builds these automatically.
        options.build_getters = false;
        options.build_constructor = false;
        options.build_hash_code = false;
        options.build_equals = false;
        options.build_to_string = false;
        Ok(())
    }

    fn class_added(&self,
                   _fields: &Vec<m::JavaField>,
                   _class_type: &ClassType,
                   class: &mut ClassSpec)
                   -> Result<()> {
        class.push_annotation(&self.data);
        Ok(())
    }
}
