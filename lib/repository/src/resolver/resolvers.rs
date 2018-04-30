//! Multiple resolvers with combined result.

use core::errors::Result;
use core::{Resolved, ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage};

pub struct Resolvers {
    resolvers: Vec<Box<Resolver>>,
}

impl Resolvers {
    pub fn new(resolvers: Vec<Box<Resolver>>) -> Resolvers {
        Resolvers {
            resolvers: resolvers,
        }
    }
}

impl Resolver for Resolvers {
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Option<Resolved>> {
        for resolver in &mut self.resolvers.iter_mut() {
            if let Some(resolved) = resolver.resolve(package)? {
                return Ok(Some(resolved));
            }
        }

        Ok(None)
    }

    fn resolve_by_prefix(&mut self, package: &RpPackage) -> Result<Vec<ResolvedByPrefix>> {
        let mut out = Vec::new();

        for resolver in &mut self.resolvers.iter_mut() {
            out.extend(resolver.resolve_by_prefix(package)?);
        }

        Ok(out)
    }

    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>> {
        let mut out = Vec::new();

        for resolver in &mut self.resolvers.iter_mut() {
            out.extend(resolver.resolve_packages()?);
        }

        Ok(out)
    }
}
