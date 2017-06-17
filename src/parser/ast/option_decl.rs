use super::*;

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<AstLoc<Value>>,
}
