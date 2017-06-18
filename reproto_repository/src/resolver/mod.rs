mod paths;
mod filesystem;
mod resolvers;

use errors::*;
use reproto_core::*;
pub use self::filesystem::*;
pub use self::paths::*;
pub use self::resolvers::*;
use std::path::PathBuf;
pub(crate) use super::errors::*;
pub(crate) use super::metadata::Metadata;

pub trait Resolver {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<PathBuf>>;
}
