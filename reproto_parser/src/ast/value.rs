use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'input> {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(String),
    Type(RpType),
    Instance(AstLoc<Instance<'input>>),
    Constant(AstLoc<RpName>),
    Array(Vec<AstLoc<Value<'input>>>),
}

impl<'input> IntoModel for Value<'input> {
    type Output = RpValue;

    fn into_model(self, path: &Path) -> Result<RpValue> {
        let out = match self {
            Value::String(string) => RpValue::String(string),
            Value::Number(number) => RpValue::Number(number),
            Value::Boolean(boolean) => RpValue::Boolean(boolean),
            Value::Identifier(identifier) => RpValue::Identifier(identifier),
            Value::Type(ty) => RpValue::Type(ty),
            Value::Instance(instance) => RpValue::Instance(instance.into_model(path)?),
            Value::Constant(name) => RpValue::Constant(name.into_model(path)?),
            Value::Array(inner) => RpValue::Array(inner.into_model(path)?),
        };

        Ok(out)
    }
}
