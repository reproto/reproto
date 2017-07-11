use super::*;

#[derive(Debug, Clone, Serialize)]
pub enum RpObject {
    Instance(Loc<RpInstance>),
    Constant(Loc<RpName>),
}
