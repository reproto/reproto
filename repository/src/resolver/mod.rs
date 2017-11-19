mod paths;
mod resolvers;

pub use self::paths::Paths;
pub use self::resolvers::Resolvers;
use core::{Object, RpRequiredPackage, Version};
use errors::*;
use std::fmt;

/// A resolved package.
#[derive(Debug)]
pub struct Resolved {
    pub version: Option<Version>,
    pub object: Box<Object>,
}

impl Resolved {
    /// Build a resolved object from a tuple pair.
    pub fn from_pair(pair: (Option<Version>, Box<Object>)) -> Resolved {
        let (version, object) = pair;

        Resolved {
            version: version,
            object: object,
        }
    }
}

impl fmt::Display for Resolved {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref version) = self.version {
            write!(fmt, "{}-{}", self.object, version)
        } else {
            self.object.fmt(fmt)
        }
    }
}

/// Trait that translates a required package into a set of versions and objects.
pub trait Resolver {
    /// Resolve the specified request.
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Vec<Resolved>>;
}
