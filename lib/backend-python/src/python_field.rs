use core::{RpModifier, RpType};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PythonField<'a> {
    pub modifier: &'a RpModifier,
    pub ty: &'a RpType,
    pub name: &'a str,
    pub ident: Rc<String>,
    pub safe_ident: Rc<String>,
}

impl<'a> PythonField<'a> {
    pub fn with_safe_ident(self, safe_ident: String) -> PythonField<'a> {
        Self {
            safe_ident: Rc::new(safe_ident),
            ..self
        }
    }
}
