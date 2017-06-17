use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpMatchMember {
    pub condition: RpLoc<RpMatchCondition>,
    pub value: RpLoc<RpValue>,
}
