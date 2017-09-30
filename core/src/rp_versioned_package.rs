use super::Version;
use super::rp_package::RpPackage;
use std::fmt;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpVersionedPackage {
    pub package: RpPackage,
    pub version: Option<Version>,
}

impl RpVersionedPackage {
    pub fn new(package: RpPackage, version: Option<Version>) -> RpVersionedPackage {
        RpVersionedPackage {
            package: package,
            version: version,
        }
    }

    pub fn into_package<F>(&self, version_fn: F) -> RpPackage
    where
        F: FnOnce(&Version) -> String,
    {
        let mut parts = Vec::new();

        parts.extend(self.package.parts.iter().cloned());

        if let Some(ref version) = self.version {
            parts.push(version_fn(version));
        }

        RpPackage::new(parts)
    }
}

impl fmt::Display for RpVersionedPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.package)?;

        if let Some(ref version) = self.version {
            write!(f, "@{}", version)?;
        }

        Ok(())
    }
}
