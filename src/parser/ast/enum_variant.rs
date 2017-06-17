use super::*;

#[derive(Debug)]
pub struct EnumVariant {
    pub name: AstLoc<String>,
    pub comment: Vec<String>,
    pub arguments: Vec<AstLoc<Value>>,
    pub ordinal: Option<AstLoc<Value>>,
}
