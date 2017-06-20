use super::*;

#[derive(Debug)]
pub struct RpFile {
    pub package_decl: RpLoc<RpPackageDecl>,
    pub options: Options,
    pub uses: Vec<RpLoc<RpUseDecl>>,
    pub decls: Vec<RpLoc<RpDecl>>,
}
