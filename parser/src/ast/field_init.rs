use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit<'input> {
    pub name: Loc<&'input str>,
    pub value: Loc<Value<'input>>,
}

impl<'input> IntoModel for FieldInit<'input> {
    type Output = RpFieldInit;

    fn into_model(self) -> Result<RpFieldInit> {
        let field_init = RpFieldInit {
            name: self.name.into_model()?,
            value: self.value.into_model()?,
        };

        Ok(field_init)
    }
}
