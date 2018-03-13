//! Value of models

use super::{Loc, RpEnumOrdinal, RpNumber};
use errors::{Error, Result};
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RpValue {
    String(String),
    Number(RpNumber),
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
            Identifier(ref identifier) => Ok(identifier),
            _ => Err("expected identifier".into()),
        }
    }

    pub fn as_string(&self) -> Result<&str> {
        use self::RpValue::*;

        match *self {
            String(ref string) => Ok(string),
            _ => Err("expected string".into()),
        }
    }

    pub fn into_ordinal(self) -> Result<RpEnumOrdinal> {
        let ordinal = match self {
            RpValue::String(value) => RpEnumOrdinal::String(value),
            _ => return Err(Error::new("Not an ordinal")),
        };

        Ok(ordinal)
    }
}

impl fmt::Display for RpValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            RpValue::String(_) => "<string>",
            RpValue::Number(_) => "<number>",
            RpValue::Identifier(_) => "<identifier>",
            RpValue::Array(_) => "<array>",
        };

        write!(f, "{}", out)
    }
}
