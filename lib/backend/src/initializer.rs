use errors::Result;

pub trait Initializer {
    type Options;

    fn initialize(&self, _: &mut Self::Options) -> Result<()> {
        Ok(())
    }
}

/// A vector of listeners is a valid listener.
impl<O> Initializer for Vec<Box<Initializer<Options = O>>> {
    type Options = O;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        for listeners in self {
            listeners.initialize(options)?;
        }

        Ok(())
    }
}
