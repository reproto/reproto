use backend::IntoBytes;
use backend::errors::*;
use genco::{Python, Tokens};
use python_compiler::PythonCompiler;

pub struct PythonFileSpec<'element>(pub Tokens<'element, Python<'element>>);

impl<'element> Default for PythonFileSpec<'element> {
    fn default() -> Self {
        PythonFileSpec(Tokens::new())
    }
}

impl<'element> IntoBytes<PythonCompiler<'element>> for PythonFileSpec<'element> {
    fn into_bytes(self, _: &PythonCompiler<'element>) -> Result<Vec<u8>> {
        let out = self.0.join_line_spacing().to_file()?;
        Ok(out.into_bytes())
    }
}
