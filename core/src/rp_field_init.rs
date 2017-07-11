use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct RpFieldInit {
    pub name: Loc<String>,
    pub value: Loc<RpValue>,
}
