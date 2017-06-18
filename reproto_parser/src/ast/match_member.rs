use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct MatchMember {
    pub condition: AstLoc<MatchCondition>,
    pub value: AstLoc<Value>,
}

impl IntoModel for MatchMember {
    type Output = RpMatchMember;

    fn into_model(self, path: &Path) -> Result<RpMatchMember> {
        let member = RpMatchMember {
            condition: self.condition.into_model(path)?,
            value: self.value.into_model(path)?,
        };

        Ok(member)
    }
}
