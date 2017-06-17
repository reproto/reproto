use parser::ast;
use super::*;
use super::errors::*;
use super::into_model::IntoModel;

#[derive(Debug, Clone, Serialize)]
pub struct RpMatchMember {
    pub condition: RpLoc<RpMatchCondition>,
    pub value: RpLoc<RpValue>,
}

impl IntoModel for ast::MatchMember {
    type Output = RpMatchMember;

    fn into_model(self, pos: &RpPos) -> Result<RpMatchMember> {
        let member = RpMatchMember {
            condition: self.condition.into_model(pos)?,
            value: self.value.into_model(pos)?,
        };

        Ok(member)
    }
}
