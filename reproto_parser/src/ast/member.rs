use super::*;

#[derive(Debug)]
pub enum Member<'a> {
    Field(Field<'a>),
    Code(String, Vec<String>),
    Option(AstLoc<OptionDecl>),
    Match(MatchDecl),
}
