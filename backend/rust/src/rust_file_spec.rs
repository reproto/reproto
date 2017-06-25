use super::*;

pub struct RustFileSpec(pub FileSpec);

impl<'a> Collecting<'a> for RustFileSpec {
    type Processor = RustCompiler<'a>;

    fn new() -> Self {
        RustFileSpec(FileSpec::new())
    }

    fn into_bytes(self, _: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();
        self.0.format(&mut out)?;
        Ok(out.into_bytes())
    }
}
