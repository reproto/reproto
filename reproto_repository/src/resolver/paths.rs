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

    fn parse_stem<'a>(&self, stem: &'a str) -> Result<(&'a str, Option<Version>)> {
        let mut it = stem.splitn(2, '-');

        if let (Some(name_base), Some(name_version)) = (it.next(), it.next()) {
            let version = Version::parse(name_version).map_err(|_| format!("bad version"))?;

            return Ok((name_base, Some(version)));
        }

        Ok((stem, None))
    }

    pub fn find_versions(&self,
                         path: &Path,
                         base: &str,
                         version_req: Option<&VersionReq>)
                         -> Result<Vec<(Option<Version>, PathBuf)>> {
        let mut files = Vec::new();

        for e in fs::read_dir(path)? {
            let p = e?.path();

            // only look for files
            if !p.is_file() {
                continue;
            }

            if p.extension().map(|ext| ext != EXT).unwrap_or(true) {
                continue;
            }

            if let Some(stem) = p.file_stem().and_then(::std::ffi::OsStr::to_str) {
                let (name_base, version) = self.parse_stem(stem)
                    .map_err(|m| format!("{}: {}", p.display(), m))?;

                if name_base != base {
                    continue;
                }

                if let Some(version_req) = version_req {
                    if let Some(version) = version {
                        if version_req.matches(&version) {
                            files.push((Some(version), p.clone()));
                        }

                        continue;
                    }

                    if *version_req == VersionReq::any() {
                        files.push((None, p.clone()));
                        continue;
                    }
                } else {
                    files.push((version, p.clone()));
                }
            }
        }

        Ok(files)
    }
}

impl Resolver for Paths {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<(Option<Version>, PathBuf)>> {
        let mut files = Vec::new();
        let version_req = package.version_req.as_ref();

        for path in &self.paths {
            let mut path: PathBuf = path.to_owned();
            let mut it = package.package.parts.iter().peekable();

            while let Some(step) = it.next() {
                if it.peek().is_none() {
                    if path.is_dir() {
                        files.extend(self.find_versions(&path, step, version_req)?);
                    }

                    break;
                }

                path = path.join(step);
            }
        }

        Ok(files)
    }
}
