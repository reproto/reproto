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

    pub fn find_versioned(&self,
                          path: &Path,
                          version_req: Option<&VersionReq>)
                          -> Result<Vec<(Option<Version>, PathBuf)>> {
        let mut files = Vec::new();

        for e in fs::read_dir(path)? {
            let p = e?.path();

            if p.extension().map(|ext| ext != EXT).unwrap_or(true) {
                continue;
            }

            // match if file stem is a valid version.
            if let Some(ref version) = p.file_stem().and_then(::std::ffi::OsStr::to_str) {
                // only include files which are valid version specs
                if let Ok(version) = Version::parse(version) {
                    if version_req.map(|req| req.matches(&version)).unwrap_or(true) {
                        files.push((Some(version), p.clone()));
                    }
                }
            }
        }

        Ok(files)
    }
}

impl Resolver for Paths {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>> {
        let mut files = Vec::new();

        for path in &self.paths {
            let mut path: PathBuf = path.to_owned();

            for part in &package.package.parts {
                path = path.join(part);
            }

            // look into package directory.
            if path.is_dir() {
                files.extend(self.find_versioned(&path, package.version_req.as_ref())?);
            }

            path.set_extension(EXT);

            // only match top-level files if there is no version requirement.
            if let None = package.version_req {
                if path.is_file() {
                    files.push((None, path.clone()));
                }
            }
        }

        Ok(files)
    }
}
