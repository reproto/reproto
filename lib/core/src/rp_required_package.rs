//! A package requirement

use super::{RpPackage, VersionReq};
use errors::*;
use std::fmt;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpRequiredPackage {
    pub package: RpPackage,
    pub version_req: VersionReq,
}

impl RpRequiredPackage {
    pub fn new(package: RpPackage, version_req: VersionReq) -> RpRequiredPackage {
        RpRequiredPackage {
            package: package,
            version_req: version_req,
        }
    }

    /// Parse the package requirement from a string.
    pub fn parse(input: &str) -> Result<RpRequiredPackage> {
        let mut it = input.splitn(2, '@').into_iter();

        let package = it.next().map(RpPackage::parse).unwrap_or_else(
            RpPackage::empty,
        );

        let version_req = if let Some(version) = it.next() {
            VersionReq::parse(version).map_err(|e| {
                format!("bad version: {}: {}", e, version)
            })?
        } else {
            VersionReq::any()
        };

        Ok(RpRequiredPackage::new(package, version_req))
    }
}

impl fmt::Display for RpRequiredPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.package, self.version_req)
    }
}
