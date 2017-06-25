use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpByTypeMatch {
    pub variable: RpLoc<RpMatchVariable>,
    pub instance: RpLoc<RpValue>,
}
