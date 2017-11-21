//! The file spec collecting changes.

use backend::IntoBytes;
use backend::errors::*;
use genco::{Rust, Tokens};
use rust_compiler::RustCompiler;

pub struct RustFileSpec<'a>(pub Tokens<'a, Rust<'a>>);

impl<'processor> Default for RustFileSpec<'processor> {
    fn default() -> Self {
        RustFileSpec(Tokens::new())
    }
}

impl<'processor> IntoBytes<RustCompiler<'processor>> for RustFileSpec<'processor> {
    fn into_bytes(self, _: &RustCompiler<'processor>) -> Result<Vec<u8>> {
        let out = self.0.join_line_spacing().to_file()?;
        Ok(out.into_bytes())
    }
}
