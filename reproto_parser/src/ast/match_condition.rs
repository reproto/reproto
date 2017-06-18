use super::*;
use super::errors::*;

#[derive(Debug)]
pub enum MatchCondition {
    /// Match a specific value.
    Value(AstLoc<Value>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(AstLoc<MatchVariable>),
}

impl IntoModel for MatchCondition {
    type Output = RpMatchCondition;

    fn into_model(self, path: &Path) -> Result<RpMatchCondition> {
        let match_condition = match self {
            MatchCondition::Value(value) => RpMatchCondition::Value(value.into_model(path)?),
            MatchCondition::Type(ty) => RpMatchCondition::Type(ty.into_model(path)?),
        };

        Ok(match_condition)
    }
}
