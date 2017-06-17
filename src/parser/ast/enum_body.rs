use super::*;

#[derive(Debug)]
pub struct EnumBody {
    pub name: String,
    pub comment: Vec<String>,
    pub variants: Vec<AstLoc<EnumVariant>>,
    pub members: Vec<AstLoc<Member>>,
}
