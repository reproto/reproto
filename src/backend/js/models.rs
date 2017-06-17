pub use core::*;

#[derive(Clone)]
pub struct JsField<'a> {
    pub modifier: &'a RpModifier,
    pub ty: &'a RpType,
    pub name: &'a str,
    pub ident: String,
}

impl<'a> JsField<'a> {
    pub fn with_ident(self, ident: String) -> JsField<'a> {
        JsField {
            modifier: self.modifier,
            ty: self.ty,
            name: self.name,
            ident: ident,
        }
    }
}
