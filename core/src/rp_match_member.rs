use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpMatchMember {
    pub condition: Loc<RpMatchCondition>,
    pub value: Loc<RpValue>,
}
