use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct RpInstance {
    pub name: RpName,
    pub arguments: Loc<Vec<Loc<RpFieldInit>>>,
}
