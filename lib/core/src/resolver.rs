use errors::Result;
use std::fmt;
use {RpPackage, RpRequiredPackage, RpVersionedPackage, Source, Version};

/// A resolved package.
#[derive(Debug)]
pub struct Resolved {
    /// Version of object found.
    pub version: Option<Version>,
    /// Source found.
    pub source: Source,
}

/// A resolved package.
#[derive(Debug)]
pub struct ResolvedByPrefix {
    /// Package object belongs to.
    pub package: RpVersionedPackage,
    /// Source found.
    pub source: Source,
}

impl Resolved {
    /// Build a resolved object from a tuple pair.
    pub fn from_pair(pair: (Option<Version>, Source)) -> Resolved {
        let (version, source) = pair;

        Resolved { version, source }
    }
}

impl fmt::Display for Resolved {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref version) = self.version {
            write!(fmt, "{}-{}", self.source, version)
        } else {
            self.source.fmt(fmt)
        }
    }
}

/// Trait that translates a required package into a set of versions and objects.
pub trait Resolver {
    /// Resolve the specified request.
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Vec<Resolved>>;

    /// Resolve by prefix.
    fn resolve_by_prefix(&mut self, package: &RpPackage) -> Result<Vec<ResolvedByPrefix>>;

    /// Find packages to build.
    /// This will internally use the `resolve_by_prefix` function, but is conditional on if a given
    /// resolver can be used to automatically locate buildable packages.
    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>>;
}

pub struct EmptyResolver;

impl Resolver for EmptyResolver {
    fn resolve(&mut self, _package: &RpRequiredPackage) -> Result<Vec<Resolved>> {
        Ok(vec![])
    }

    fn resolve_by_prefix(&mut self, _package: &RpPackage) -> Result<Vec<ResolvedByPrefix>> {
        Ok(vec![])
    }

    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>> {
        Ok(vec![])
    }
}
