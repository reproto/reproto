use super::*;

pub trait Listeners {
    fn configure(&self, _: &mut RustOptions) -> Result<()> {
        Ok(())
    }
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    fn configure(&self, options: &mut RustOptions) -> Result<()> {
        for listeners in self {
            listeners.configure(options)?;
        }

        Ok(())
    }
}
