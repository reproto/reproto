use super::Objects;
use core::errors::*;
use core::{self, Resolved, ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage,
           RpVersionedPackage, Source, Version};
use index::{Deployment, Index};
use sha256::to_sha256;
use update::Update;

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

    pub fn update(&self) -> Result<Vec<Update>> {
        let mut updates = Vec::new();
        updates.extend(self.index.update()?);
        updates.extend(self.objects.update()?);
        Ok(updates)
    }

    /// Publish the given package and version.
    pub fn publish(
        &mut self,
        object: &Source,
        package: &RpPackage,
        version: &Version,
        force: bool,
    ) -> Result<()> {
        if !self.index.get_deployments(package, version)?.is_empty() {
            if !force {
                return Err(format!("{}@{}: already published", package, version).into());
            } else {
                info!("{}@{}: already published (forced)", package, version);
            }
        }

        let checksum = to_sha256(object.read()?)?;

        self.objects
            .put_object(&checksum, &mut object.read()?, force)?;
        self.index.put_version(&checksum, package, version, force)?;

        Ok(())
    }

    /// Get all deployments in this repository.
    pub fn all(&self, package: &RpPackage) -> Result<Vec<Deployment>> {
        self.index.all(package)
    }

    /// Get the object for the specific deployment.
    pub fn get_object(&mut self, deployment: &Deployment) -> Result<Option<Source>> {
        // NOTE: objects from repositories are _always_ read-only.
        self.objects
            .get_object(&deployment.object)
            .map(|s| s.map(|s| s.with_read_only(true)))
    }
}

impl Resolver for Repository {
    fn resolve(&mut self, package: &RpRequiredPackage) -> core::errors::Result<Vec<Resolved>> {
        let mut out = Vec::new();

        let deployments = self.index.resolve(&package.package, &package.range)?;

        for deployment in deployments {
            if let Some(source) = self.get_object(&deployment)? {
                out.push(Resolved {
                    version: Some(deployment.version),
                    source,
                });
            } else {
                return Err(format!("missing object: {}", deployment.object).into());
            }
        }

        Ok(out)
    }

    fn resolve_by_prefix(
        &mut self,
        package: &RpPackage,
    ) -> core::errors::Result<Vec<ResolvedByPrefix>> {
        let mut out = Vec::new();

        let deployments = self.index.resolve_by_prefix(&package)?;

        for (deployment, package) in deployments {
            if let Some(source) = self.get_object(&deployment)? {
                let package = RpVersionedPackage::new(package, Some(deployment.version));
                out.push(ResolvedByPrefix { package, source });
            } else {
                return Err(format!("missing object: {}", deployment.object).into());
            }
        }

        Ok(out)
    }

    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>> {
        Ok(vec![])
    }
}
