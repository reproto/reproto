use super::*;
use super::errors::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(tag = "type", content="value", rename_all="snake_case")]
pub enum RpValue {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(String),
    Array(Vec<Loc<RpValue>>),
}

impl RpValue {
    pub fn as_match_kind(&self) -> RpMatchKind {
        match *self {
            RpValue::String(_) => RpMatchKind::String,
            RpValue::Number(_) => RpMatchKind::Number,
            RpValue::Boolean(_) => RpMatchKind::Boolean,
            RpValue::Identifier(_) => RpMatchKind::Any,
            RpValue::Array(_) => RpMatchKind::Array,
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        match *self {
            RpValue::String(ref string) => Ok(string),
            _ => Err("not a string".into()),
        }
    }
}

impl ::std::fmt::Display for RpValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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
