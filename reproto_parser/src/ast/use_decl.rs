use super::*;

#[derive(Debug)]
pub struct UseDecl {
    pub package: AstLoc<RpPackage>,
    pub version_req: Option<AstLoc<VersionReq>>,
    pub alias: Option<String>,
}
