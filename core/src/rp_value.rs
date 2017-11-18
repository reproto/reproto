//! Value of models

use super::{Loc, RpEnumOrdinal, RpNumber};
use errors::*;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RpValue {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(String),
    Array(Vec<Loc<RpValue>>),
}

impl RpValue {
    pub fn as_str(&self) -> Result<&str> {
        use self::RpValue::*;

        match *self {
            String(ref string) => Ok(string),
            _ => Err("not a string".into()),
        }
    }

    pub fn as_identifier(&self) -> Result<&str> {
        use self::RpValue::*;

        match *self {
            String(ref string) => Ok(string),
            Identifier(ref identifier) => Ok(identifier),
            _ => Err("unsupported identifier kind".into()),
        }
    }

    pub fn to_ordinal(self) -> Result<RpEnumOrdinal> {
        let ordinal = match self {
            RpValue::String(value) => RpEnumOrdinal::String(value),
            _ => return Err(ErrorKind::InvalidOrdinal.into()),
        };

        Ok(ordinal)
    }
}

impl fmt::Display for RpValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            RpValue::String(_) => "<string>",
            RpValue::Number(_) => "<number>",
            RpValue::Boolean(_) => "<boolean>",
            RpValue::Identifier(_) => "<identifier>",
            RpValue::Array(_) => "<array>",
        };

        write!(f, "{}", out)
    }
}
