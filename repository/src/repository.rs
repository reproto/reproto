use errors::*;
use index::Index;
use objects::Objects;
use reproto_core::*;
use resolver::Resolver;
use std::path::PathBuf;

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
}

impl Resolver for Repository {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>> {
        Ok(vec![])
    }
}
