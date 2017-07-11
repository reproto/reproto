use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'input> {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(String),
    Type(RpType),
    Instance(Loc<Instance<'input>>),
    Constant(Loc<RpName>),
    Array(Vec<Loc<Value<'input>>>),
}

impl<'input> IntoModel for Value<'input> {
    type Output = RpValue;

    fn into_model(self) -> Result<RpValue> {
        let out = match self {
            Value::String(string) => RpValue::String(string),
            Value::Number(number) => RpValue::Number(number),
            Value::Boolean(boolean) => RpValue::Boolean(boolean),
            Value::Identifier(identifier) => RpValue::Identifier(identifier),
            Value::Type(ty) => RpValue::Type(ty),
            Value::Instance(instance) => RpValue::Instance(instance.into_model()?),
            Value::Constant(name) => RpValue::Constant(name.into_model()?),
            Value::Array(inner) => RpValue::Array(inner.into_model()?),
        };

        Ok(out)
    }
}
