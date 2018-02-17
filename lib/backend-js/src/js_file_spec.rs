//! File spec collecting results from backends

use backend::IntoBytes;
use core::errors::*;
use genco::{JavaScript, Tokens};
use js_compiler::JsCompiler;

pub struct JsFileSpec<'el>(pub Tokens<'el, JavaScript<'el>>);

impl<'el> Default for JsFileSpec<'el> {
    fn default() -> Self {
        JsFileSpec(Tokens::new())
    }
}

impl<'el> IntoBytes<JsCompiler<'el>> for JsFileSpec<'el> {
    fn into_bytes(self, _: &JsCompiler<'el>) -> Result<Vec<u8>> {
        let out = self.0.join_line_spacing().to_file()?;
        Ok(out.into_bytes())
    }
}
