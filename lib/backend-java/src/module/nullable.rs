use crate::codegen::Configure;

pub struct Module;

impl Module {
    pub fn initialize(self, e: Configure) {
        e.options.nullable = true;
    }
}
