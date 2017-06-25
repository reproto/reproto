use super::*;

pub struct JsFileSpec(pub FileSpec);

impl<'a> Collecting<'a> for JsFileSpec {
    type Processor = JsCompiler<'a>;

    fn new() -> Self {
        JsFileSpec(FileSpec::new())
    }

    fn into_bytes(self, _: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();
        self.0.format(&mut out)?;
        Ok(out.into_bytes())
    }
}
