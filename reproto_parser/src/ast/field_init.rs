use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: AstLoc<String>,
    pub value: AstLoc<Value>,
}

impl IntoModel for FieldInit {
    type Output = RpFieldInit;

    fn into_model(self, path: &Path) -> Result<RpFieldInit> {
        let field_init = RpFieldInit {
            name: self.name.into_model(path)?,
            value: self.value.into_model(path)?,
        };

        Ok(field_init)
    }
}
