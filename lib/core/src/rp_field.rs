//! Data Models for fields

use super::{RpModifier, RpType};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RpField {
    pub modifier: RpModifier,
    pub name: String,
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

    pub fn ident(&self) -> &str {
        &self.name
    }

    pub fn name(&self) -> &str {
        self.field_as.as_ref().unwrap_or(&self.name)
    }

    pub fn display(&self) -> String {
        self.name.to_owned()
    }
}
