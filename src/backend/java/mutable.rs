use super::processor;

use errors::*;

pub struct Module {
}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl processor::Listeners for Module {
    fn configure(&self, options: &mut processor::ProcessorOptions) -> Result<()> {
        options.immutable = false;
        Ok(())
    }
}
