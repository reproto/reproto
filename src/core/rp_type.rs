use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpType {
    Double,
    Float,
    Signed { size: Option<usize> },
    Unsigned { size: Option<usize> },
    Boolean,
    String,
    Bytes,
    Any,
    Name { name: RpName },
    Array { inner: Box<RpType> },
    Map {
        key: Box<RpType>,
        value: Box<RpType>,
    },
}

impl ::std::fmt::Display for RpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RpType::Double => write!(f, "double"),
            RpType::Float => write!(f, "float"),
            RpType::Signed { ref size } => {
                if let Some(size) = *size {
                    write!(f, "signed/{}", size)
                } else {
                    write!(f, "signed")
                }
            }
            RpType::Unsigned { ref size } => {
                if let Some(size) = *size {
                    write!(f, "unsigned/{}", size)
                } else {
                    write!(f, "unsigned")
                }
            }
            RpType::Boolean => write!(f, "boolean"),
            RpType::String => write!(f, "string"),
            RpType::Name { ref name } => {
                if let Some(ref used) = name.prefix {
                    write!(f, "{}::{}", used, name.parts.join("."))
                } else {
                    write!(f, "{}", name.parts.join("."))
                }
            }
            RpType::Array { ref inner } => write!(f, "[{}]", inner),
            RpType::Map { ref key, ref value } => write!(f, "{{{}: {}}}", key, value),
            RpType::Any => write!(f, "any"),
            RpType::Bytes => write!(f, "bytes"),
        }
    }
}
