use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

    pub fn into_type_id(&self, name: RpName) -> RpTypeId {
        RpTypeId::new(self.clone(), name)
    }

    pub fn into_package<F>(&self, version_fn: F) -> RpPackage
        where F: FnOnce(&Version) -> String
    {
        let mut parts = self.package.parts.clone();

        if let Some(ref version) = self.version {
            parts.push(version_fn(version));
        }

        RpPackage::new(parts)
    }
}

impl ::std::fmt::Display for RpVersionedPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.package)?;

        if let Some(ref version) = self.version {
            write!(f, "@{}", version)?;
        }

        Ok(())
    }
}
