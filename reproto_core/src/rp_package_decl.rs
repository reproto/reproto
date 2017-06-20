use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpPackageDecl {
    pub package: RpPackage,
    pub version: Option<RpLoc<Version>>,
}
