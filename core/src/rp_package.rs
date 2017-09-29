use super::rp_versioned_package::RpVersionedPackage;
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RpPackage {
    pub parts: Vec<String>,
}

impl RpPackage {
    pub fn new(parts: Vec<String>) -> RpPackage {
        RpPackage { parts: parts }
    }

    pub fn empty() -> RpPackage {
        RpPackage { parts: vec![] }
    }

    pub fn join_versioned(&self, other: &RpVersionedPackage) -> RpVersionedPackage {
        let mut parts = self.parts.clone();
        parts.extend(other.package.parts.clone());
        RpVersionedPackage::new(RpPackage::new(parts), other.version.clone())
    }

    pub fn join(&self, other: &RpPackage) -> RpPackage {
        let mut parts = self.parts.clone();
        parts.extend(other.parts.clone());
        RpPackage::new(parts)
    }
}

impl fmt::Display for RpPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}
