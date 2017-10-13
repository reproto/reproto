use core::{RpModifier, RpType};
use std::rc::Rc;

#[derive(Clone)]
pub struct JsField<'a> {
    pub modifier: &'a RpModifier,
    pub ty: &'a RpType,
    pub name: &'a str,
    pub ident: Rc<String>,
}

impl<'a> JsField<'a> {
    pub fn with_ident(self, ident: String) -> JsField<'a> {
        JsField {
            modifier: self.modifier,
            ty: self.ty,
            name: self.name,
            ident: Rc::new(ident),
        }
    }
}
