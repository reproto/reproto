use super::*;
use super::errors::*;

#[derive(Debug, Clone)]
pub struct OptionDecl<'input> {
    pub name: &'input str,
    pub values: Vec<RpLoc<Value<'input>>>,
}

impl<'input> IntoModel for OptionDecl<'input> {
    type Output = RpOptionDecl;

    fn into_model(self) -> Result<RpOptionDecl> {
        let decl = RpOptionDecl {
            name: self.name.to_owned(),
            values: self.values.into_model()?,
        };

        Ok(decl)
    }
}
