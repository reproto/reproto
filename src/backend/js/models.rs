pub use backend::models::*;

#[derive(Clone)]
pub struct JsField {
    pub modifier: RpModifier,
    pub ty: RpType,
    pub name: String,
    pub ident: String,
}
