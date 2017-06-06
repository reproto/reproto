use super::into_model::IntoModel;
use super::rp_loc::{RpLoc, RpPos};

#[derive(Debug, PartialEq, Clone)]
pub enum RpValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Type(RpType),
    Instance(RpLoc<Instance>),
    Constant(RpLoc<RpName>),
    Array(Vec<RpLoc<RpValue>>),
}

impl ::std::fmt::Display for RpValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let out = match *self {
            RpValue::String(_) => "<string>",
            RpValue::Number(_) => "<number>",
            RpValue::Boolean(_) => "<boolean>",
            RpValue::Identifier(_) => "<identifier>",
            RpValue::Type(_) => "<type>",
            RpValue::Instance(_) => "<instance>",
            RpValue::Constant(_) => "<constant>",
            RpValue::Array(_) => "<array>",
        };

        write!(f, "{}", out)
    }
}

impl IntoModel for ast::RpValue {
    type Output = RpValue;

    fn into_model(self, pos: &RpPos) -> Result<RpValue> {
        let value = match self {
            ast::RpValue::String(string) => RpValue::String(string),
            ast::RpValue::Number(number) => RpValue::Number(number),
            ast::RpValue::Boolean(boolean) => RpValue::Boolean(boolean),
            ast::RpValue::Identifier(identifier) => RpValue::Identifier(identifier),
            ast::RpValue::Type(ty) => RpValue::Type(ty),
            ast::RpValue::Instance(instance) => RpValue::Instance(instance.into_model(pos)?),
            ast::RpValue::Constant(constant) => {
                RpValue::Constant(constant.with_prefix(pos.0.clone()))
            }
            ast::RpValue::Array(values) => RpValue::Array(values.into_model(pos)?),
        };

        Ok(value)
    }
}
