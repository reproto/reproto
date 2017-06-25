use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RpVersionedPackage {
    pub package: Option<RpPackage>,
    pub version: Option<Version>,
}

impl RpVersionedPackage {
    pub fn new(package: Option<RpPackage>, version: Option<Version>) -> RpVersionedPackage {
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
        let mut parts = Vec::new();

        if let Some(ref package) = self.package {
            parts.extend(package.parts.iter().map(Clone::clone));
        }

        if let Some(ref version) = self.version {
            parts.push(version_fn(version));
        }

        RpPackage::new(parts)
    }
}

impl ::std::fmt::Display for RpVersionedPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if let Some(ref package) = self.package {
            write!(f, "{}", package)?;
        } else {
            write!(f, "*empty*")?;
        }

        if let Some(ref version) = self.version {
            write!(f, "@{}", version)?;
        }

        Ok(())
    }
}
