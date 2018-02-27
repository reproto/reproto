use core::{RpModifier, RpType};
use genco::Cons;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PythonField<'el> {
    pub modifier: RpModifier,
    pub ty: RpType,
    pub name: Cons<'el>,
    pub ident: Rc<String>,
    pub safe_ident: Rc<String>,
    pub comment: Vec<Cons<'el>>,
}

impl<'el> PythonField<'el> {
    pub fn with_safe_ident(self, safe_ident: String) -> PythonField<'el> {
        Self {
            safe_ident: Rc::new(safe_ident),
            ..self
        }
    }
}
