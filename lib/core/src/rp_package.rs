use super::RpVersionedPackage;
use serde;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpPackage {
    pub parts: Vec<String>,
}

impl RpPackage {
    pub fn new(parts: Vec<String>) -> RpPackage {
        RpPackage { parts: parts }
    }

    /// Parse a package from a string.
    pub fn parse(input: &str) -> RpPackage {
        RpPackage::new(input.split(".").map(ToOwned::to_owned).collect())
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

    /// Join with the given part.
    pub fn join_part<S: AsRef<str>>(mut self, other: S) -> RpPackage {
        self.parts.push(other.as_ref().to_string());
        self
    }
}

impl fmt::Display for RpPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

impl serde::Serialize for RpPackage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for RpPackage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RpPackageVisitor;

        impl<'de> serde::de::Visitor<'de> for RpPackageVisitor {
            type Value = RpPackage;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a SemVer version as a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(RpPackage::parse(v))
            }
        }

        deserializer.deserialize_str(RpPackageVisitor)
    }
}
