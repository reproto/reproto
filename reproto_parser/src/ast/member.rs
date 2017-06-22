use super::*;

#[derive(Debug)]
pub enum Member<'input> {
    Field(Field<'input>),
    Code(&'input str, Vec<String>),
    Option(AstLoc<'input, OptionDecl<'input>>),
    Match(MatchDecl<'input>),
}
