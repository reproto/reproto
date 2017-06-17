use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpEnumVariant {
    pub name: RpLoc<String>,
    pub comment: Vec<String>,
    pub arguments: Vec<RpLoc<RpValue>>,
    pub ordinal: u32,
}
