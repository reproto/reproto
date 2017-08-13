use errors::*;
use index::Index;
use object::Object;
use objects::Objects;
use reproto_core::*;
use resolver::Resolver;
use sha256::to_sha256;

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

    pub fn publish<O>(
        &mut self,
        object: O,
        package: &RpPackage,
        version: &Version,
        force: bool,
    ) -> Result<()>
    where
        O: AsRef<Object>,
    {
        if !self.index.get_deployments(package, version)?.is_empty() {
            if !force {
                return Err(format!("{}@{}: already published", package, version).into());
            } else {
                info!("{}@{}: already published (forced)", package, version);
            }
        }

        let object = object.as_ref();
        let checksum = to_sha256(object.read()?)?;

        self.objects.put_object(
            &checksum,
            &mut object.read()?,
            force,
        )?;
        self.index.put_version(&checksum, package, version, force)?;

        Ok(())
    }
}

impl Resolver for Repository {
    fn resolve(
        &mut self,
        package: &RpRequiredPackage,
    ) -> Result<Vec<(Option<Version>, Box<Object>)>> {
        let mut out = Vec::new();

        let deployments = self.index.resolve(
            &package.package,
            package.version_req.as_ref(),
        )?;

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
