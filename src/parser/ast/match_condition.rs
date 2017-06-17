use super::*;

#[derive(Debug)]
pub enum MatchCondition {
    /// Match a specific value.
    Value(AstLoc<Value>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(AstLoc<MatchVariable>),
}
