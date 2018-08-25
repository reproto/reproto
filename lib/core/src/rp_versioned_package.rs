//! A versioned package declaration

use errors::Result;
use rp_package::Parts;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use {AsPackage, RpPackage, RpPackageFormat, Version};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpVersionedPackage {
    pub package: RpPackage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

impl AsPackage for RpVersionedPackage {
    /// Convert into a package by piping the version through the provided function.
    fn try_as_package<'a>(&'a self) -> Result<Cow<'a, RpPackage>> {
        Ok(Cow::Owned(self.to_package(|v| v.to_string())))
    }

    fn prefix_with(self, prefix: RpPackage) -> Self {
        prefix.join_versioned(self)
    }
}

impl RpVersionedPackage {
    pub fn new(package: RpPackage, version: Option<Version>) -> Self {
        Self { package, version }
    }

    /// Create an empty versioned package.
    pub fn empty() -> Self {
        Self {
            package: RpPackage::empty(),
            version: None,
        }
    }

    /// Check if this package starts with another package.
    pub fn starts_with(&self, other: &RpPackage) -> bool {
        self.package.starts_with(other)
    }

    /// Convert into a package by piping the version through the provided function.
    pub fn to_package<V>(&self, version_fn: V) -> RpPackage
    where
        V: FnOnce(&Version) -> String,
    {
        let mut parts = Vec::new();

        parts.extend(self.package.parts().cloned());

        if let Some(ref version) = self.version {
            parts.push(version_fn(version));
        }

        RpPackage::new(parts)
    }

    /// Convert to a package without a version.
    pub fn without_version(self) -> Self {
        Self {
            package: self.package,
            version: None,
        }
    }

    /// Replace all keyword components in this package.
    pub fn with_replacements(self, keywords: &HashMap<String, String>) -> Self {
        Self {
            package: self.package.with_replacements(keywords),
            ..self
        }
    }

    /// Apply the given naming policy to this package.
    pub fn with_naming<N>(self, naming: N) -> Self
    where
        N: Fn(&str) -> String,
    {
        Self {
            package: self.package.with_naming(naming),
            ..self
        }
    }

    /// Iterate over the parts of the package.
    pub fn parts(&self) -> Parts {
        self.package.parts()
    }
}

impl fmt::Display for RpVersionedPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        RpPackageFormat(&self.package, self.version.as_ref()).fmt(f)
    }
}
