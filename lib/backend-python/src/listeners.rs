use super::*;

pub trait Listeners {
    fn configure(&self, _: &mut PythonOptions) -> Result<()> {
        Ok(())
    }
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    fn configure(&self, processor: &mut PythonOptions) -> Result<()> {
        for listeners in self {
            listeners.configure(processor)?;
        }

        Ok(())
    }
}
