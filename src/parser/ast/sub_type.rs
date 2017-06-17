use super::*;

/// Sub-types in interface declarations.
#[derive(Debug)]
pub struct SubType {
    pub name: String,
    pub comment: Vec<String>,
    pub members: Vec<AstLoc<Member>>,
}
