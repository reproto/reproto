//! The file spec collecting changes.

use backend::IntoBytes;
use compiler::Compiler;
use core::errors::Result;
use core::RpPackage;
use genco::{Dart, Tokens};

pub struct DartFileSpec<'a>(pub Tokens<'a, Dart<'a>>);

impl<'el> Default for DartFileSpec<'el> {
    fn default() -> Self {
        DartFileSpec(Tokens::new())
    }
}

impl<'el> IntoBytes<Compiler<'el>> for DartFileSpec<'el> {
    fn into_bytes(self, _: &Compiler<'el>, _: &RpPackage) -> Result<Vec<u8>> {
        let out = self.0.join_line_spacing().to_file()?;
        Ok(out.into_bytes())
    }
}
