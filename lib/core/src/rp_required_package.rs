//! A package requirement

use errors::Result;
use std::fmt;
use {Range, RpPackage};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpRequiredPackage {
    pub package: RpPackage,
    pub range: Range,
}

impl RpRequiredPackage {
    pub fn new(package: RpPackage, range: Range) -> RpRequiredPackage {
        RpRequiredPackage { package, range }
    }

    /// Parse the package requirement from a string.
    pub fn parse(input: &str) -> Result<RpRequiredPackage> {
        let mut it = input.splitn(2, '@');

        let package = it
            .next()
            .map(RpPackage::parse)
            .unwrap_or_else(RpPackage::empty);

        let range = if let Some(version) = it.next() {
            Range::parse(version).map_err(|e| format!("bad version: {}: {}", e, version))?
        } else {
            Range::any()
        };

        Ok(RpRequiredPackage::new(package, range))
    }
}

impl fmt::Display for RpRequiredPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.range.matches_any() {
            write!(f, "{}", self.package)
        } else {
            write!(f, "{} {}", self.package, self.range)
        }
    }
}
