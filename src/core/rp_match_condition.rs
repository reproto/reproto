use parser::ast;
use super::*;
use super::errors::*;
use super::into_model::IntoModel;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpMatchCondition {
    /// Match a specific value.
    Value(RpLoc<RpValue>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(RpLoc<RpMatchVariable>),
}

impl IntoModel for ast::MatchCondition {
    type Output = RpMatchCondition;

    fn into_model(self, pos: &RpPos) -> Result<RpMatchCondition> {
        let match_condition = match self {
            ast::MatchCondition::Value(value) => RpMatchCondition::Value(value.into_model(pos)?),
            ast::MatchCondition::Type(ty) => RpMatchCondition::Type(ty.into_model(pos)?),
        };

        Ok(match_condition)
    }
}
