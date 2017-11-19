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

use core::{Object, PathObject, RpPackage, RpRequiredPackage, Version, VersionReq};
use errors::*;
use resolver::Resolver;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

const EXT: &str = "reproto";

pub struct Paths {
    /// Paths to perform lookups in.
    paths: Vec<PathBuf>,
    /// Entries which are locally published.
    published: HashMap<RpPackage, Version>,
}

impl Paths {
    pub fn new(paths: Vec<PathBuf>, published: HashMap<RpPackage, Version>) -> Paths {
        Paths {
            paths: paths,
            published: published,
        }
    }

    fn parse_stem<'a>(&self, stem: &'a str) -> Result<(&'a str, Option<Version>)> {
        let mut it = stem.splitn(2, '-');

        if let (Some(name_base), Some(name_version)) = (it.next(), it.next()) {
            let version = Version::parse(name_version).map_err(
                |_| format!("bad version"),
            )?;

            return Ok((name_base, Some(version)));
        }

        Ok((stem, None))
    }

    /// Finds the published version from most to least specific package.
    pub fn find_published_version(&self, package: &RpPackage) -> Option<&Version> {
        if let Some(version) = self.published.get(package) {
            return Some(version);
        }

        let mut it = package.parts.iter();

        while let Some(_) = it.next_back() {
            let package = RpPackage::new(it.as_slice().to_vec());

            if let Some(version) = self.published.get(&package) {
                return Some(version);
            }
        }

        None
    }

    /// Find any matching versions.
    ///
    /// TODO: Make `version_req` not use `Option`.
    pub fn find_versions(
        &self,
        path: &Path,
        base: &str,
        package: &RpPackage,
        version_req: &VersionReq,
    ) -> Result<Vec<(Option<Version>, Box<Object>)>> {
        let mut files: BTreeMap<Option<Version>, Box<Object>> = BTreeMap::new();

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
                let (name_base, version) = self.parse_stem(stem).map_err(|m| {
                    format!("{}: {}", p.display(), m)
                })?;

                if name_base != base {
                    continue;
                }

                let version = version.or_else(|| self.find_published_version(package).cloned());

                if let Some(version) = version {
                    if version_req.matches(&version) {
                        let object = PathObject::new(None, &p);
                        files.insert(Some(version), Box::new(object));
                    }

                    continue;
                }

                if version_req.matches_any() {
                    let object = PathObject::new(None, &p);
                    files.insert(None, Box::new(object));
                }
            }
        }

        Ok(files.into_iter().collect())
    }
}

impl Resolver for Paths {
    fn resolve(
        &mut self,
        package: &RpRequiredPackage,
    ) -> Result<Vec<(Option<Version>, Box<Object>)>> {
        let mut files = Vec::new();

        for path in &self.paths {
            let mut path: PathBuf = path.to_owned();
            let mut it = package.package.parts.iter().peekable();

            while let Some(step) = it.next() {
                if it.peek().is_none() {
                    if path.is_dir() {
                        files.extend(self.find_versions(
                            &path,
                            step,
                            &package.package,
                            &package.version_req,
                        )?);
                    }

                    break;
                }

                path = path.join(step);
            }
        }

        Ok(files)
    }
}
