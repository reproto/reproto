mod paths;
mod resolvers;

pub use self::paths::Paths;
pub use self::resolvers::Resolvers;
use core::{Object, RpRequiredPackage, Version};
use errors::*;

pub trait Resolver {
    fn resolve(
        &mut self,
        package: &RpRequiredPackage,
    ) -> Result<Vec<(Option<Version>, Box<Object>)>>;
}
