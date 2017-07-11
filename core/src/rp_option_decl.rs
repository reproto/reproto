use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpOptionDecl {
    pub name: String,
    pub values: Vec<Loc<RpValue>>,
}
