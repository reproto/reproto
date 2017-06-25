use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct RpFieldInit {
    pub name: RpLoc<String>,
    pub value: RpLoc<RpValue>,
}
