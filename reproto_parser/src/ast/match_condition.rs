use super::*;
use super::errors::*;

#[derive(Debug)]
pub enum MatchCondition<'input> {
    /// Match a specific value.
    Value(AstLoc<'input, Value<'input>>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(AstLoc<'input, MatchVariable<'input>>),
}

impl<'input> IntoModel for MatchCondition<'input> {
    type Output = RpMatchCondition;

    fn into_model(self) -> Result<RpMatchCondition> {
        let match_condition = match self {
            MatchCondition::Value(value) => RpMatchCondition::Value(value.into_model()?),
            MatchCondition::Type(ty) => RpMatchCondition::Type(ty.into_model()?),
        };

        Ok(match_condition)
    }
}
