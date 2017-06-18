use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RpPackage {
    pub parts: Vec<String>,
}

impl RpPackage {
    pub fn new(parts: Vec<String>) -> RpPackage {
        RpPackage { parts: parts }
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

    pub fn into_type_id(&self, version: Option<Version>, name: RpName) -> RpTypeId {
        RpTypeId::new(RpVersionedPackage::new(self.clone(), version), name)
    }
}

impl ::std::fmt::Display for RpPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}
