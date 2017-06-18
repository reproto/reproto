use errors::*;
use reproto_core::RpPackage;
pub use self::filesystem::*;
pub(crate) use super::errors::*;
pub(crate) use super::metadata::Metadata;

mod filesystem;

pub trait Resolver {
    fn resolve(&self, package: &RpPackage) -> Result<Option<Metadata>>;
}
