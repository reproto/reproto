use super::VersionReq;
use super::rp_package::RpPackage;
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

impl fmt::Display for RpRequiredPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.package)?;

        if let Some(ref version_req) = self.version_req {
            write!(f, "@{}", version_req)?;
        }

        Ok(())
    }
}
