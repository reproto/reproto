use super::*;

#[derive(Debug)]
pub struct TupleBody {
    pub name: String,
    pub comment: Vec<String>,
    pub members: Vec<AstLoc<Member>>,
}
