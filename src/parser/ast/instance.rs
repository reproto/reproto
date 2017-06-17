use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub name: RpName,
    pub arguments: AstLoc<Vec<AstLoc<FieldInit>>>,
}
