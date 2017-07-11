use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpByValueMatch {
    pub object: Loc<RpObject>,
}
