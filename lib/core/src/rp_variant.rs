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
    /// Get the identifier of the variant.
    pub fn ident(&self) -> &str {
        self.ident.as_str()
    }

    /// Get the ordinal value of the variant.
    pub fn ordinal(&self) -> &str {
        use self::RpEnumOrdinal::*;

        match self.ordinal {
            String(ref string) => string.as_str(),
            Generated => self.ident(),
        }
    }
}
