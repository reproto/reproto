use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RpRequiredPackage {
    pub package: RpPackage,
    pub version_req: Option<VersionReq>,
}

impl RpRequiredPackage {
    pub fn new(package: RpPackage, version_req: Option<VersionReq>) -> RpRequiredPackage {
        RpRequiredPackage {
            package: package,
            version_req: version_req,
        }
    }
}

impl ::std::fmt::Display for RpRequiredPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.package)?;

        if let Some(ref version_req) = self.version_req {
            write!(f, "@[{}]", version_req)?;
        }

        Ok(())
    }
}
