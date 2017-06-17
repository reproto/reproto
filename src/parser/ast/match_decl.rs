use super::*;

#[derive(Debug)]
pub struct MatchDecl {
    pub members: Vec<AstLoc<MatchMember>>,
}
