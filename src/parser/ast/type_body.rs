use super::*;

#[derive(Debug)]
pub struct TypeBody {
    pub name: String,
    pub comment: Vec<String>,
    pub members: Vec<AstLoc<Member>>,
}
