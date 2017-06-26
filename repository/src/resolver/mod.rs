mod paths;
mod resolvers;

use errors::*;
use reproto_core::*;
pub use self::paths::*;
pub use self::resolvers::*;
use std::path::PathBuf;
pub(crate) use super::errors::*;

pub trait Resolver {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>>;
}
