use super::*;

#[derive(Debug)]
pub struct UseDecl {
    pub package: AstLoc<RpPackage>,
    pub alias: Option<String>,
}
