use super::*;

#[derive(Debug)]
pub struct InterfaceBody {
    pub name: String,
    pub comment: Vec<String>,
    pub members: Vec<AstLoc<Member>>,
    pub sub_types: Vec<AstLoc<SubType>>,
}
