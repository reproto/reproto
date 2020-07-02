//! Value of models

use crate::errors::Result;
use crate::{Diagnostics, Flavor, RpName, RpNumber, Spanned, Translate, Translator};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    bound = "F::Package: Serialize"
)]
pub enum RpValue<F: 'static>
where
    F: Flavor,
{
    String(String),
    Number(RpNumber),
    Identifier(String),
    Array(Vec<Spanned<RpValue<F>>>),
    Name(Spanned<RpName<F>>),
}

impl<F: 'static> RpValue<F>
where
    F: Flavor,
{
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

    /// Convert into identifier.
    pub fn into_identifier(self) -> Result<String> {
        use self::RpValue::*;

        match self {
            Identifier(string) => Ok(string),
            _ => Err("expected identifier".into()),
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

impl<F: 'static> fmt::Display for RpValue<F>
where
    F: Flavor,
{
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
            RpValue::Name(ref name) => name.fmt(f),
        }
    }
}

impl<F: 'static, T> Translate<T> for RpValue<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = RpValue<T::Target>;

    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpValue<T::Target>> {
        use self::RpValue::*;

        let out = match self {
            String(string) => String(string),
            Number(number) => Number(number),
            Identifier(string) => Identifier(string),
            Array(array) => Array(array.translate(diag, translator)?),
            Name(name) => Name(name.translate(diag, translator)?),
        };

        Ok(out)
    }
}
