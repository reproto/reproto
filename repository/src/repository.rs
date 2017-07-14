use errors::*;
use index::Index;
use objects::Objects;
use reproto_core::*;
use resolver::Resolver;
use sha256::to_sha256;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct Repository {
    index: Box<Index>,
    objects: Box<Objects>,
}

impl Repository {
    pub fn new(index: Box<Index>, objects: Box<Objects>) -> Repository {
        Repository {
            index: index,
            objects: objects,
        }
    }

    pub fn update(&self) -> Result<()> {
        self.index.update()?;
        self.objects.update()?;
        Ok(())
    }

    pub fn publish<P>(&mut self, source: &P, package: &RpPackage, version: &Version) -> Result<()>
        where P: AsRef<Path>
    {
        let source = source.as_ref();

        if !self.index.get_deployments(package, version)?.is_empty() {
            return Err(format!("{}@{}: already published", package, version).into());
        }

        let checksum = to_sha256(File::open(source)?)?;

        {
            let mut f = File::open(source)?;
            self.objects.put_object(&checksum, &mut f)?;
        }

        self.index.put_version(&checksum, package, version)?;
        Ok(())
    }
}

impl Resolver for Repository {
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>> {
        let mut out = Vec::new();

        let deployments = self.index.resolve(&package.package, package.version_req.as_ref())?;

        for deployment in deployments {
            if let Some(path) = self.objects.get_object(&deployment.object)? {
                out.push((Some(deployment.version), path));
            } else {
                return Err(format!("missing object: {}", deployment.object).into());
            }
        }

        Ok(out)
    }
}
