//! # Default paths-based resolver
//!
//! Resolves packages based on a set of paths.
//!
//! These paths have the following structure:
//!
//! * `<root>/<package>/<last>.reproto`
//! * `<root>/<package>/<last>/<version>.reproto`
//!
//! The second form is only used when a version requirement is present.

use std::fs;
use std::path::{Path, PathBuf};
use super::*;

const EXT: &str = "reproto";

pub struct Paths {
    paths: Vec<PathBuf>,
}

impl Paths {
    pub fn new(paths: Vec<PathBuf>) -> Paths {
        Paths { paths: paths }
    }

    pub fn find_versioned(&self, path: &Path, version_req: &VersionReq) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for e in fs::read_dir(path)? {
            let p = e?.path();

            if p.extension().map(|ext| ext != EXT).unwrap_or(true) {
                continue;
            }

            // match if file stem is a valid version.
            if let Some(Ok(v)) = p.file_stem().and_then(|s| s.to_str()).map(Version::parse) {
                if version_req.matches(&v) {
                    files.push(p.clone());
                }
            }
        }

        Ok(files)
    }
}

impl Resolver for Paths {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for path in &self.paths {
            let mut path: PathBuf = path.to_owned();

            for part in &package.package.parts {
                path = path.join(part);
            }

            // if there is a version requirement, find versioned files in sub-directory.
            if let Some(ref version_req) = package.version_req {
                if path.is_dir() {
                    files.extend(self.find_versioned(&path, version_req)?);
                }
            }

            path.set_extension(EXT);

            if path.is_file() {
                files.push(path.clone());
            }
        }

        Ok(files)
    }
}
