//! Literal code segments

#[derive(Debug, Clone, Serialize)]
pub struct RpCode {
    pub context: String,
    pub lines: Vec<String>,
}
