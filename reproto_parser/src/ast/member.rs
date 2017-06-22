use super::*;

#[derive(Debug)]
pub enum Member<'input> {
    Field(Field<'input>),
    Code(&'input str, Vec<String>),
    Option(AstLoc<OptionDecl<'input>>),
    Match(MatchDecl<'input>),
}
