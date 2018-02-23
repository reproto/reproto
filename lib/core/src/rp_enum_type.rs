//! Model for enum types

use super::{RpField, RpModifier, RpType, RpValue};
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub enum RpEnumType {
    String,
    Generated,
}

impl RpEnumType {
    pub fn is_assignable_from(&self, value: &RpValue) -> bool {
        use self::RpEnumType::*;

        match (self, value) {
            (&String, &RpValue::String(_)) => true,
            _ => false,
        }
    }

    pub fn as_type(&self) -> RpType {
        use self::RpEnumType::*;

        match *self {
            String | Generated => RpType::String,
        }
    }

    pub fn as_field(&self) -> RpField {
        RpField {
            modifier: RpModifier::Required,
            ident: String::from("value"),
            safe_ident: None,
            comment: vec![],
            ty: self.as_type(),
            field_as: None,
        }
    }
}

impl fmt::Display for RpEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpEnumType::*;

        match *self {
            String => write!(f, "string"),
            Generated => write!(f, "generated"),
        }
    }
}
