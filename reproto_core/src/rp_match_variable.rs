use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RpMatchVariable {
    pub name: String,
    #[serde(rename="type")]
    pub ty: RpType,
}
