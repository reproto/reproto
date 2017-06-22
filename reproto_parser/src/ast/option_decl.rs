use super::*;
use super::errors::*;

#[derive(Debug, Clone)]
pub struct OptionDecl<'input> {
    pub name: &'input str,
    pub values: Vec<AstLoc<Value<'input>>>,
}

impl<'input> IntoModel for OptionDecl<'input> {
    type Output = RpOptionDecl;

    fn into_model(self, path: &Path) -> Result<RpOptionDecl> {
        let decl = RpOptionDecl {
            name: self.name.to_owned(),
            values: self.values.into_model(path)?,
        };

        Ok(decl)
    }
}
