use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpByTypeMatch {
    pub variable: Loc<RpMatchVariable>,
    pub instance: Loc<RpValue>,
}
