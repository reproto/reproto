use reproto_core::RpPackage;

use super::Resolver;
use super::errors::*;
use super::metadata::Metadata;

pub struct Repository<'a> {
    resolvers: Vec<&'a Resolver>,
}

impl<'a> Repository<'a> {
    pub fn new(resolvers: Vec<&'a Resolver>) -> Repository<'a> {
        Repository { resolvers: resolvers }
    }

    // Resolve metadata from any backend
    pub fn resolve(&mut self, package: &RpPackage) -> Result<Vec<(&Resolver, Metadata)>> {
        let mut out = Vec::new();

        for resolver in &self.resolvers {
            if let Some(metadata) = resolver.resolve(package)? {
                out.push((*resolver, metadata));
            }
        }

        Ok(out)
    }
}
