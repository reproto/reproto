use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit<'input> {
    pub name: AstLoc<&'input str>,
    pub value: AstLoc<Value<'input>>,
}

impl<'input> IntoModel for FieldInit<'input> {
    type Output = RpFieldInit;

    fn into_model(self, path: &Path) -> Result<RpFieldInit> {
        let field_init = RpFieldInit {
            name: self.name.into_model(path)?,
            value: self.value.into_model(path)?,
        };

        Ok(field_init)
    }
}
