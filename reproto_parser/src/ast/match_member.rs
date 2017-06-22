use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct MatchMember<'input> {
    pub condition: AstLoc<MatchCondition<'input>>,
    pub value: AstLoc<Value<'input>>,
}

impl<'input> IntoModel for MatchMember<'input> {
    type Output = RpMatchMember;

    fn into_model(self, path: &Path) -> Result<RpMatchMember> {
        let member = RpMatchMember {
            condition: self.condition.into_model(path)?,
            value: self.value.into_model(path)?,
        };

        Ok(member)
    }
}
