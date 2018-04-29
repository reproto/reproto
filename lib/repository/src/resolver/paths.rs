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

use core::errors::Result;
use core::{Range, Resolved, ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage, Source,
           Version};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

const EXT: &str = "reproto";

pub struct Paths {
    /// Paths to perform lookups in.
    paths: Vec<PathBuf>,
    /// Entries which are locally published.
    published: HashMap<RpPackage, Version>,
    /// Do we support automatic packages?
    automatic_packages: bool,
}

impl Paths {
    pub fn new(
        paths: Vec<PathBuf>,
        published: HashMap<RpPackage, Version>,
        automatic_packages: bool,
    ) -> Paths {
        Paths {
            paths,
            published,
            automatic_packages,
        }
    }

    fn parse_stem<'a>(&self, stem: &'a str) -> Result<(&'a str, Option<Version>)> {
        let mut it = stem.splitn(2, '-');

        if let (Some(name_base), Some(name_version)) = (it.next(), it.next()) {
            let version = Version::parse(name_version).map_err(|e| format!("bad version: {}", e))?;

            return Ok((name_base, Some(version)));
        }

        Ok((stem, None))
    }

    /// Finds the published version from most to least specific package.
    pub fn find_published_version(&self, package: &RpPackage) -> Option<&Version> {
        if let Some(version) = self.published.get(package) {
            return Some(version);
        }

        let mut it = package.parts();

        while let Some(_) = it.next_back() {
            let package = RpPackage::new(it.as_slice().to_vec());

            if let Some(version) = self.published.get(&package) {
                return Some(version);
            }
        }

        None
    }

    /// Find any matching versions.
    pub fn find_by_range(
        &self,
        path: &Path,
        base: &str,
        package: &RpPackage,
        range: &Range,
    ) -> Result<Vec<Resolved>> {
        let mut files: BTreeMap<Option<Version>, Source> = BTreeMap::new();

        let entries = fs::read_dir(path)
            .map_err(|e| format!("failed to read directory: {}: {}", path.display(), e))?;

        for e in entries {
            let path = e?.path();

            // only look for files
            if !path.is_file() {
                continue;
            }

            if path.extension().map(|ext| ext != EXT).unwrap_or(true) {
                continue;
            }

            let stem = match path.file_stem() {
                Some(stem) => stem.to_str()
                    .ok_or_else(|| format!("non-utf8 file name: {}", path.display()))?,
                None => continue,
            };

            let (name_base, version) = self.parse_stem(stem)
                .map_err(|e| format!("bad file: {}: {}", path.display(), e.display()))?;

            if name_base != base {
                continue;
            }

            let version = version.or_else(|| self.find_published_version(package).cloned());

            // versioned matches by requirement.
            if let Some(version) = version {
                if range.matches(&version) {
                    files.insert(Some(version), Source::from_path(&path));
                }

                continue;
            }

            // unversioned only matches by wildcard.
            if range.matches_any() {
                files.insert(None, Source::from_path(&path));
            }
        }

        Ok(files.into_iter().map(Resolved::from_pair).collect())
    }
}

impl Resolver for Paths {
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Vec<Resolved>> {
        let mut files = Vec::new();

        for path in &self.paths {
            let mut path: PathBuf = path.to_owned();
            let mut it = package.package.parts().peekable();

            while let Some(base) = it.next() {
                if it.peek().is_none() {
                    if path.is_dir() {
                        files.extend(self.find_by_range(
                            &path,
                            base,
                            &package.package,
                            &package.range,
                        )?);
                    }

                    break;
                }

                path = path.join(base);
            }
        }

        Ok(files)
    }

    fn resolve_by_prefix(&mut self, package: &RpPackage) -> Result<Vec<ResolvedByPrefix>> {
        let mut out = Vec::new();
        let mut queue = VecDeque::new();

        for path in &self.paths {
            if !path.is_dir() {
                continue;
            }

            let path = package
                .parts()
                .fold(path.to_owned(), |p, part| p.join(part));

            let last = match package.last() {
                Some(last) => last,
                None => {
                    queue.push_back((package.clone(), path.to_owned()));
                    continue;
                }
            };

            let path = path.parent()
                .ok_or_else(|| format!("path does not have a parent: {}", path.display()))?;

            // definitely not here.
            if !path.is_dir() {
                continue;
            }

            let entries = fs::read_dir(path)
                .map_err(|e| format!("failed to read directory: {}: {}", path.display(), e))?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                if path.extension().map(|ext| ext != EXT).unwrap_or(true) {
                    debug!("skipping wrong file extension: {}", path.display());
                    continue;
                }

                let stem = match path.file_stem() {
                    Some(stem) => stem.to_str()
                        .ok_or_else(|| format!("non-utf8 file name: {}", path.display()))?,
                    None => continue,
                };

                let (name, version) = self.parse_stem(&stem)
                    .map_err(|e| format!("bad file: {}: {}", path.display(), e.display()))?;

                if name != last {
                    continue;
                }

                let source = Source::from_path(&path);

                out.push(ResolvedByPrefix {
                    package: package.clone(),
                    version,
                    source,
                });
            }
        }

        while let Some((package, path)) = queue.pop_front() {
            if !path.is_dir() {
                continue;
            }

            let entries = fs::read_dir(&path)
                .map_err(|e| format!("failed to read directory: {}: {}", path.display(), e))?;

            for entry in entries {
                let entry = entry?;

                let path = entry.path();

                if path.is_file() {
                    if path.extension().map(|e| e != EXT).unwrap_or(true) {
                        debug!("skipping wrong file extension: {}", path.display());
                        continue;
                    }

                    let stem = match path.file_stem() {
                        Some(stem) => stem.to_str()
                            .ok_or_else(|| format!("non-utf8 file name: {}", path.display()))?,
                        None => continue,
                    };

                    let (name, version) = self.parse_stem(stem)
                        .map_err(|e| format!("bad file: {}: {}", path.display(), e.display()))?;
                    let package = package.clone().join_part(name);
                    let source = Source::from_path(&path);

                    out.push(ResolvedByPrefix {
                        package,
                        version,
                        source,
                    });

                    continue;
                }

                let base = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| format!("non-utf8 file name: {}", path.display()))?;

                if path.is_dir() {
                    let package = package.clone().join_part(base);
                    queue.push_back((package, path));
                }
            }
        }

        Ok(out)
    }

    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>> {
        if !self.automatic_packages {
            return Ok(vec![]);
        }

        self.resolve_by_prefix(&RpPackage::empty())
    }
}
