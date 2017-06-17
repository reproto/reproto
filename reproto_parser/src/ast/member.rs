use super::*;

#[derive(Debug)]
pub enum Member {
    Field(Field),
    Code(String, Vec<String>),
    Option(AstLoc<OptionDecl>),
    Match(MatchDecl),
}
