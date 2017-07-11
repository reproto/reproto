use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum RpObject {
    Instance(Loc<RpInstance>),
    Constant(Loc<RpName>),
}
