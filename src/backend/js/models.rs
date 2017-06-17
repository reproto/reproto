pub use core::*;

#[derive(Clone)]
pub struct JsField {
    pub modifier: RpModifier,
    pub ty: RpType,
    pub name: String,
    pub ident: String,
}

impl JsField {
    pub fn with_ident(self, ident: String) -> JsField {
        JsField {
            modifier: self.modifier,
            ty: self.ty,
            name: self.name,
            ident: ident,
        }
    }
}
