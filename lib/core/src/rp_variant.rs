//! Variant in an enum

use super::{Loc, RpEnumOrdinal, RpName};

#[derive(Debug, Clone, Serialize)]
pub struct RpVariant {
    pub name: RpName,
    pub ident: Loc<String>,
    pub comment: Vec<String>,
    pub ordinal: RpEnumOrdinal,
}

impl RpVariant {
    pub fn ordinal(&self) -> &str {
        use self::RpEnumOrdinal::*;

        match self.ordinal {
            String(ref string) => string.as_str(),
            Generated => self.ident.as_str(),
        }
    }
}
