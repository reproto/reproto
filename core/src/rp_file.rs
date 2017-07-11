use super::*;

#[derive(Debug)]
pub struct RpFile {
    pub options: Options,
    pub uses: Vec<Loc<RpUseDecl>>,
    pub decls: Vec<Loc<RpDecl>>,
}
