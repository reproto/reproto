mod paths;
mod resolvers;

use core::{RpRequiredPackage, Version};
use errors::*;
pub use self::paths::Paths;
pub use self::resolvers::Resolvers;
use std::path::PathBuf;

pub trait Resolver {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>>;
}
