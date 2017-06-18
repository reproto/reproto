use super::*;

pub struct Resolvers {
    resolvers: Vec<Box<Resolver>>,
}

impl Resolvers {
    pub fn new(resolvers: Vec<Box<Resolver>>) -> Resolvers {
        Resolvers { resolvers: resolvers }
    }
}

impl Resolver for Resolvers {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<PathBuf>> {
        let mut out = Vec::new();

        for resolver in &self.resolvers {
            out.extend(resolver.resolve(package)?);
        }

        Ok(out)
    }
}
