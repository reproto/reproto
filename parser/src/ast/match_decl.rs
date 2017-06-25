use super::*;

#[derive(Debug)]
pub struct MatchDecl<'input> {
    pub members: Vec<RpLoc<MatchMember<'input>>>,
}
