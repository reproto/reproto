use super::*;

#[derive(Debug, Clone)]
pub struct RpUseDecl {
    pub package: Loc<RpPackage>,
    pub version_req: Option<Loc<VersionReq>>,
    pub alias: Option<String>,
}
