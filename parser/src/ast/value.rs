use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'input> {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(&'input str),
    Array(Vec<Loc<Value<'input>>>),
}

impl<'input> IntoModel for Value<'input> {
    type Output = RpValue;

    fn into_model(self) -> Result<RpValue> {
        let out = match self {
            Value::String(string) => RpValue::String(string),
            Value::Number(number) => RpValue::Number(number),
            Value::Boolean(boolean) => RpValue::Boolean(boolean),
            Value::Identifier(identifier) => RpValue::Identifier(identifier.to_owned()),
            Value::Array(inner) => RpValue::Array(inner.into_model()?),
        };

        Ok(out)
    }
}
