use backend::*;
use super::processor::*;

pub struct Module {
}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Listeners for Module {
    fn configure(&self, options: &mut ProcessorOptions) -> Result<()> {
        options.nullable = true;
        Ok(())
    }
}
