//! Data Models for fields

use Flavor;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RpField<F: 'static>
where
    F: Flavor,
{
    /// Is the field required.
    pub required: bool,
    /// Mangled identifier, taking target-specific keywords into account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_ident: Option<String>,
    /// Original identifier used to specify the field.
    pub ident: String,
    /// Field comments.
    pub comment: Vec<String>,
    #[serde(rename = "type")]
    pub ty: F::Type,
    /// Alias of field in JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_as: Option<String>,
}

impl<F: 'static> RpField<F>
where
    F: Flavor,
{
    pub fn new<S: AsRef<str>>(ident: S, ty: F::Type) -> Self {
        RpField {
            required: true,
            safe_ident: None,
            ident: ident.as_ref().to_string(),
            comment: Vec::new(),
            ty: ty,
            field_as: None,
        }
    }

    pub fn is_optional(&self) -> bool {
        !self.required
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Get the keyword-safe identifier.
    ///
    /// This will be the identifier escaped to avoid any target-language keywords.
    pub fn safe_ident(&self) -> &str {
        self.safe_ident.as_ref().unwrap_or(&self.ident)
    }

    /// Change the safe identifier.
    pub fn with_safe_ident<S: AsRef<str>>(self, safe_ident: S) -> RpField<F> {
        Self {
            safe_ident: Some(safe_ident.as_ref().to_string()),
            ..self
        }
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
