//! Model for enum types

use RpValue;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub enum RpEnumType {
    String,
}

impl RpEnumType {
    pub fn is_assignable_from(&self, value: &RpValue) -> bool {
        use self::RpEnumType::*;

        match (self, value) {
            (&String, &RpValue::String(_)) => true,
            _ => false,
        }
    }
}

impl fmt::Display for RpEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpEnumType::*;

        match *self {
            String => write!(f, "string"),
        }
    }
}
