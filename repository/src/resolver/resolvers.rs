use core::{RpRequiredPackage, Version};
use errors::*;
use resolver::Resolver;
use std::path::PathBuf;

pub struct Resolvers {
    resolvers: Vec<Box<Resolver>>,
}

impl Resolvers {
    pub fn new(resolvers: Vec<Box<Resolver>>) -> Resolvers {
        Resolvers { resolvers: resolvers }
    }
}

impl Resolver for Resolvers {
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>> {
        let mut out = Vec::new();

        for resolver in &mut self.resolvers.iter_mut() {
            out.extend(resolver.resolve(package)?);
        }

        Ok(out)
    }
}
