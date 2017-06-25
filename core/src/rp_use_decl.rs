use super::*;

#[derive(Debug, Clone)]
pub struct RpUseDecl {
    pub package: RpLoc<RpPackage>,
    pub version_req: Option<RpLoc<VersionReq>>,
    pub alias: Option<String>,
}
