use core::{RpModifier, RpType};
use std::rc::Rc;

#[derive(Clone)]
pub struct JsField<'a> {
    pub modifier: &'a RpModifier,
    pub ty: &'a RpType,
    pub name: &'a str,
    pub ident: Rc<String>,
    pub safe_ident: Rc<String>,
}

impl<'a> JsField<'a> {
    pub fn with_ident(self, ident: String) -> JsField<'a> {
        Self {
            ident: Rc::new(ident),
            ..self
        }
    }
}
