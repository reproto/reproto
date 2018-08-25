use errors::Result;
use serde;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::result;
use std::slice;
use {AsPackage, RpVersionedPackage};

/// Iterator over parts in a package.
pub struct Parts<'a> {
    iter: slice::Iter<'a, String>,
}

impl<'a> Iterator for Parts<'a> {
    type Item = <slice::Iter<'a, String> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> DoubleEndedIterator for Parts<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a> Parts<'a> {
    /// Treat the remaining iterator as a slice.
    pub fn as_slice(&self) -> &'a [String] {
        self.iter.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpPackage {
    parts: Vec<String>,
}

impl AsPackage for RpPackage {
    fn try_as_package<'a>(&'a self) -> Result<Cow<'a, RpPackage>> {
        Ok(Cow::Borrowed(self))
    }

    fn prefix_with(self, prefix: RpPackage) -> Self {
        prefix.join_package(self)
    }
}

impl RpPackage {
    pub fn new(parts: Vec<String>) -> RpPackage {
        RpPackage { parts }
    }

    /// Get length of package.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Check if package is empty.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Parse a package from a string.
    ///
    /// Warning: This does not perform any validation that the given package only contains
    /// identifier components!
    ///
    /// * An empty string results in a package _without_ any parts.
    /// * All other strings are split on dots.
    pub fn parse(input: &str) -> RpPackage {
        if input.is_empty() {
            return Self::empty();
        }

        RpPackage::new(input.split('.').map(ToOwned::to_owned).collect())
    }

    /// Build an empty package.
    pub fn empty() -> RpPackage {
        RpPackage { parts: vec![] }
    }

    /// Join with the other package.
    pub fn join_package(mut self, other: RpPackage) -> RpPackage {
        self.parts.extend(other.parts);
        self
    }

    /// Join this package with another, versioned, package.
    pub fn join_versioned(&self, other: RpVersionedPackage) -> RpVersionedPackage {
        let mut parts = self.parts.clone();
        parts.extend(other.package.parts);
        RpVersionedPackage::new(RpPackage::new(parts), other.version)
    }

    /// Join the parts of this package with the given string.
    pub fn join(&self, separator: &str) -> String {
        self.parts.join(separator)
    }

    /// Join with the given part.
    pub fn join_part<S: AsRef<str>>(mut self, other: S) -> RpPackage {
        self.parts.push(other.as_ref().to_string());
        self
    }

    /// Check if this package starts with another package.
    pub fn starts_with(&self, other: &RpPackage) -> bool {
        if self.parts.len() < other.parts.len() {
            return false;
        }

        self.parts
            .iter()
            .zip(other.parts.iter())
            .all(|(a, b)| a == b)
    }

    /// Replace all keyword components in this package.
    pub fn with_replacements(mut self, keywords: &HashMap<String, String>) -> Self {
        for p in &mut self.parts {
            if let Some(keyword) = keywords.get(p.as_str()) {
                mem::replace(p, keyword.to_string());
            }
        }

        self
    }

    /// Apply the given naming policy to each part.
    pub fn with_naming<N>(mut self, naming: N) -> Self
    where
        N: Fn(&str) -> String,
    {
        for p in &mut self.parts {
            let new_name = naming(p);
            mem::replace(p, new_name);
        }

        self
    }

    /// Iterate over the parts in the package.
    pub fn parts(&self) -> Parts {
        Parts {
            iter: self.parts.iter(),
        }
    }

    /// Split at the last part, returning the package prefix and an optional last part if present.
    pub fn split_last(mut self) -> (RpPackage, Option<String>) {
        if self.parts.is_empty() {
            return (self, None);
        }

        let len = self.parts.len();
        let parts = self.parts.split_off(len - 1);
        let last = parts.into_iter().next();
        (RpPackage::new(self.parts), last)
    }

    /// Get the last part, if package has one.
    pub fn last(&self) -> Option<&str> {
        self.parts.last().map(|s| s.as_str())
    }
}

/// Convenience conversion, mostly used for tests.
impl<S> From<Vec<S>> for RpPackage
where
    String: From<S>,
{
    fn from(value: Vec<S>) -> Self {
        RpPackage::new(value.into_iter().map(String::from).collect())
    }
}

impl fmt::Display for RpPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

impl serde::Serialize for RpPackage {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for RpPackage {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RpPackageVisitor;

        impl<'de> serde::de::Visitor<'de> for RpPackageVisitor {
            type Value = RpPackage;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a SemVer version as a string")
            }

            fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(RpPackage::parse(v))
            }
        }

        deserializer.deserialize_str(RpPackageVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::RpPackage;

    #[test]
    fn test_split_last() {
        assert_eq!((RpPackage::empty(), None), RpPackage::empty().split_last());

        let package = RpPackage::from(vec!["foo"]);

        assert_eq!(
            (RpPackage::empty(), Some("foo".to_string())),
            package.split_last()
        );

        let package = RpPackage::from(vec!["foo", "bar"]);

        assert_eq!(
            (RpPackage::from(vec!["foo"]), Some("bar".to_string())),
            package.split_last()
        );
    }
}
