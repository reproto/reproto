//! A versioned package declaration

use super::{RpPackage, RpPackageFormat, Version};
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

    /// Convert into a package by piping the version through the provided function.
    pub fn as_package<F>(&self, version_fn: F) -> RpPackage
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

    pub fn without_version(self) -> RpVersionedPackage {
        RpVersionedPackage::new(self.package, None)
    }
}

impl fmt::Display for RpVersionedPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        RpPackageFormat(&self.package, self.version.as_ref()).fmt(f)
    }
}
