use super::*;

#[derive(Debug)]
pub struct RpFile {
    pub options: Options,
    pub uses: Vec<RpLoc<RpUseDecl>>,
    pub decls: Vec<RpLoc<RpDecl>>,
}
