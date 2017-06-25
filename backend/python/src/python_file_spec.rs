use super::*;

pub struct PythonFileSpec(pub FileSpec);

impl<'a> Collecting<'a> for PythonFileSpec {
    type Processor = PythonCompiler<'a>;

    fn new() -> Self {
        PythonFileSpec(FileSpec::new())
    }

    fn into_bytes(self, _: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();
        self.0.format(&mut out)?;
        Ok(out.into_bytes())
    }
}
