use super::*;

#[derive(Debug)]
pub struct File {
    pub package: AstLoc<RpPackage>,
    pub uses: Vec<AstLoc<UseDecl>>,
    pub decls: Vec<AstLoc<Decl>>,
}
