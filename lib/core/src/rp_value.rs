//! Value of models

use errors::Result;
use std::fmt;
use {Loc, RpNumber};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RpValue {
    String(String),
    Number(RpNumber),
    Identifier(String),
    Array(Vec<Loc<RpValue>>),
}

impl RpValue {
    /// Treat as a string.
    pub fn as_str(&self) -> Result<&str> {
        use self::RpValue::*;

        match *self {
            String(ref string) => Ok(string),
            _ => Err("not a string".into()),
        }
    }

    /// Treat as a string.
    pub fn as_number(&self) -> Result<&RpNumber> {
        use self::RpValue::*;

        match *self {
            Number(ref number) => Ok(number),
            _ => Err("not a number".into()),
        }
    }

    /// Treat as a string.
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

    /// Treat into a string.
    pub fn into_number(self) -> Result<RpNumber> {
        use self::RpValue::*;

        match self {
            Number(number) => Ok(number),
            _ => Err("not a number".into()),
        }
    }

    pub fn into_string(self) -> Result<String> {
        use self::RpValue::*;

        match self {
            String(string) => Ok(string),
            _ => Err("expected string".into()),
        }
    }

    /// Is this value a string.
    pub fn is_string(&self) -> bool {
        use self::RpValue::*;

        match *self {
            String(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for RpValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RpValue::String(ref string) => write!(f, "\"{}\"", string),
            RpValue::Number(ref number) => number.fmt(f),
            RpValue::Identifier(ref identifier) => identifier.fmt(f),
            RpValue::Array(ref values) => {
                write!(f, "[")?;

                let mut it = values.iter().peekable();

                while let Some(v) = it.next() {
                    v.fmt(f)?;

                    if it.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")?;
                Ok(())
            }
        }
    }
}
