//! Data Models for fields

use super::{RpModifier, RpType};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RpField {
    /// Modifier of the field.
    pub modifier: RpModifier,
    /// Mangled identifier, taking target-specific keywords into account.
    pub safe_ident: Option<String>,
    /// Original identifier used to specify the field.
    pub ident: String,
    /// Field comments.
    pub comment: Vec<String>,
    #[serde(rename = "type")]
    pub ty: RpType,
    /// Alias of field in JSON.
    pub field_as: Option<String>,
}

impl RpField {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }

    pub fn is_required(&self) -> bool {
        !self.is_optional()
    }

    /// Get the keyword-safe identifier.
    ///
    /// This will be the identifier escaped to avoid any target-language keywords.
    pub fn safe_ident(&self) -> &str {
        self.safe_ident.as_ref().unwrap_or(&self.ident)
    }

    /// Get the original identifier of the field.
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Get the JSON name of the field, if it differs from `ident`.
    ///
    /// TODO: Return `Option`, currently returns ident. This is a better indication whether
    /// 'renaming' should occur.
    pub fn name(&self) -> &str {
        self.field_as.as_ref().unwrap_or(&self.ident)
    }

    pub fn display(&self) -> String {
        self.name().to_owned()
    }
}
