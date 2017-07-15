mod paths;
mod resolvers;

use core::{RpRequiredPackage, Version};
use errors::*;
use object::Object;
pub use self::paths::Paths;
pub use self::resolvers::Resolvers;

pub trait Resolver {
    fn resolve(&mut self,
               package: &RpRequiredPackage)
               -> Result<Vec<(Option<Version>, Box<Object>)>>;
}
