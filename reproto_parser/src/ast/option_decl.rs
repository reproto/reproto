use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<AstLoc<Value>>,
}

impl IntoModel for OptionDecl {
    type Output = RpOptionDecl;

    fn into_model(self, pos: &RpPos) -> Result<RpOptionDecl> {
        let decl = RpOptionDecl {
            name: self.name,
            values: self.values.into_model(pos)?,
        };

        Ok(decl)
    }
}
