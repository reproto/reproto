use super::*;

#[derive(Debug)]
pub struct MatchMember {
    pub condition: AstLoc<MatchCondition>,
    pub value: AstLoc<Value>,
}
