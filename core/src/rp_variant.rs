//! Variant in an enum

use super::{Loc, RpEnumOrdinal, RpName};

#[derive(Debug, Clone, Serialize)]
pub struct RpVariant {
    pub name: RpName,
    pub local_name: Loc<String>,
    pub comment: Vec<String>,
    pub ordinal: RpEnumOrdinal,
}

impl RpVariant {
    pub fn ordinal(&self) -> &str {
        use self::RpEnumOrdinal::*;

        match self.ordinal {
            String(ref string) => string.as_str(),
            Generated => self.local_name.as_str(),
        }
    }
}
