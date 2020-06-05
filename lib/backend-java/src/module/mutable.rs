use crate::Options;

pub struct Module;

impl Module {
    pub fn initialize(self, options: &mut Options) {
        options.immutable = false;
    }
}
