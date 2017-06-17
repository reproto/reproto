use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: AstLoc<String>,
    pub value: AstLoc<Value>,
}
