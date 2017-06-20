use super::*;

#[derive(Clone)]
pub struct Field<'a> {
    pub modifier: &'a RpModifier,
    pub ty: &'a RpType,
    pub name: &'a str,
    pub ident: String,
}

impl<'a> Field<'a> {
    pub fn with_ident(self, ident: String) -> Field<'a> {
        Field {
            modifier: self.modifier,
            ty: self.ty,
            name: self.name,
            ident: ident,
        }
    }
}
