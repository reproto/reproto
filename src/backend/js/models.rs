pub use backend::models::*;

#[derive(Clone)]
pub struct JsField {
    pub modifier: Modifier,
    pub ty: Type,
    pub name: String,
    pub ident: String,
}
