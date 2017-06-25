use super::*;

pub struct Module {
}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Listeners for Module {
    fn configure(&self, options: &mut JavaOptions) -> Result<()> {
        options.nullable = true;
        Ok(())
    }
}
