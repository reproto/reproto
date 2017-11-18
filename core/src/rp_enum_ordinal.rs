//! Data Models for the final model stage stage.

#[derive(Debug, Clone, Serialize)]
pub enum RpEnumOrdinal {
    /// Value is specified expliticly.
    String(String),
    /// Value is automatically derived from the name of the variant.
    Generated,
}
