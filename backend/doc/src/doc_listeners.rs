use super::*;

pub trait DocListeners {
    fn configure(&self, _: &mut DocOptions) -> Result<()> {
        Ok(())
    }
}

impl DocListeners for Vec<Box<DocListeners>> {
    fn configure(&self, options: &mut DocOptions) -> Result<()> {
        for listener in self {
            listener.configure(options)?;
        }

        Ok(())
    }
}
