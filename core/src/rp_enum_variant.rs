use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpEnumVariant {
    pub name: Loc<String>,
    pub comment: Vec<String>,
    pub arguments: Vec<Loc<RpValue>>,
    pub ordinal: u32,
}
