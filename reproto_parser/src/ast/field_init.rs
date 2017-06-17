use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: AstLoc<String>,
    pub value: AstLoc<Value>,
}

impl IntoModel for FieldInit {
    type Output = RpFieldInit;

    fn into_model(self, pos: &RpPos) -> Result<RpFieldInit> {
        let field_init = RpFieldInit {
            name: self.name.into_model(pos)?,
            value: self.value.into_model(pos)?,
        };

        Ok(field_init)
    }
}
