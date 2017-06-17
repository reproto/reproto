use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpByValueMatch {
    pub instance: RpLoc<RpValue>,
}
