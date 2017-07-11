use super::*;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpMatchCondition {
    /// Match a specific value.
    Value(Loc<RpValue>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(Loc<RpMatchVariable>),
}
