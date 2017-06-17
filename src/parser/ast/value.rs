use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Path(RpPath),
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(String),
    Type(RpType),
    Instance(AstLoc<Instance>),
    Constant(AstLoc<RpName>),
    Array(Vec<AstLoc<Value>>),
}
