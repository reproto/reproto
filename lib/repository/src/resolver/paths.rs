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

use reproto_core::errors::{Error, Result};
use reproto_core::{
    Range, Resolved, ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage, RpVersionedPackage,
    Source, Version,
};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const EXT: &str = "reproto";
const DOT_EXT: &str = ".reproto";

pub struct Paths {
    /// Paths to perform lookups in.
    paths: Vec<PathBuf>,
    /// Entries which are locally published.
    published: HashMap<RpPackage, Version>,
}

impl Paths {
    pub fn new(paths: Vec<PathBuf>, published: HashMap<RpPackage, Version>) -> Paths {
        Paths { paths, published }
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
                Some(stem) => stem
                    .to_str()
                    .ok_or_else(|| format!("non-utf8 file name: {}", path.display()))?,
                None => continue,
            };

            let (name_base, version) = parse_stem(stem)
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
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Option<Resolved>> {
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

        Ok(files.into_iter().next_back())
    }

    fn resolve_by_prefix(&mut self, package: &RpPackage) -> Result<Vec<ResolvedByPrefix>> {
        let mut out = Vec::new();
        // contains a tuple: (package, path, search)
        // search is an optional value which designates that we are searching for a specific
        // package.
        let mut queue = VecDeque::new();

        for path in &self.paths {
            // last component needs special treatment.
            // if present, crack open parent directory and list it - it might contain both
            // leaf packages and additional sub-packages.
            let (package, last) = match package.clone().split_last() {
                (package, Some(last)) => (package, last),
                (package, None) => {
                    let path = package
                        .parts()
                        .fold(path.to_owned(), |p, part| p.join(part));

                    queue.push_back((package, path.to_owned(), None));
                    continue;
                }
            };

            let path = package
                .parts()
                .fold(path.to_owned(), |p, part| p.join(part));
            queue.push_back((package, path.to_owned(), Some(last)));
        }

        while let Some((package, path, search)) = queue.pop_front() {
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
                        log::debug!("skipping wrong file extension: {}", path.display());
                        continue;
                    }

                    let stem = match path.file_stem() {
                        Some(stem) => stem
                            .to_str()
                            .ok_or_else(|| format!("non-utf8 file name: {}", path.display()))?,
                        None => continue,
                    };

                    let (name, version) = parse_stem(stem)
                        .map_err(|e| format!("bad file: {}: {}", path.display(), e.display()))?;

                    if let Some(search) = search.as_ref() {
                        if name != *search {
                            continue;
                        }
                    }

                    let package = RpVersionedPackage::new(package.clone().join_part(name), version);
                    let source = Source::from_path(&path);

                    out.push(ResolvedByPrefix { package, source });
                    continue;
                }

                if path.is_dir() {
                    let base = entry
                        .file_name()
                        .into_string()
                        .map_err(|_| format!("non-utf8 file name: {}", path.display()))?;

                    if let Some(search) = search.as_ref() {
                        if base != *search {
                            continue;
                        }
                    }

                    let package = package.clone().join_part(base);
                    queue.push_back((package, path, None));
                    continue;
                }
            }
        }

        Ok(out)
    }

    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>> {
        self.resolve_by_prefix(&RpPackage::empty())
    }
}

/// Parse a relative path into a package.
pub fn path_to_package<P: AsRef<Path>>(path: P) -> Result<RpVersionedPackage> {
    let path = path.as_ref();

    let mut c = path.components();
    let mut last = None;

    while let Some(c) = c.next_back() {
        last = Some(match c {
            Component::Normal(last) => last,
            Component::CurDir => continue,
            part => return Err(path_part_error(path, part)),
        });

        break;
    }

    let (base, version) = match last {
        Some(last) => parse_file_name(path, last)?,
        None => return Ok(RpVersionedPackage::empty()),
    };

    let mut parts = Vec::new();

    while let Some(part) = c.next() {
        let part = match part {
            Component::Normal(part) => part,
            Component::CurDir => continue,
            part => return Err(path_part_error(path, part)),
        };

        let part = part
            .to_str()
            .ok_or_else(|| format!("non-utf8 file name: {}", path.display()))?;

        parts.push(part.to_string());
    }

    parts.push(base.to_string());

    let package = RpPackage::new(parts);
    Ok(RpVersionedPackage::new(package, version))
}

/// Construct a helpful component error.
fn path_part_error<'a>(path: &'a Path, part: Component<'a>) -> Error {
    let part = part.as_os_str();

    let part = match part.to_str() {
        Some(part) => part,
        None => {
            return Error::from(format!(
                "non-utf8 component `{}` of path `{}`",
                part.to_string_lossy(),
                path.display()
            ));
        }
    };

    Error::from(format!(
        "not a valid package component `{}` of path `{}`",
        part,
        path.display()
    ))
}

fn parse_file_name<'a>(path: &Path, name: &'a OsStr) -> Result<(&'a str, Option<Version>)> {
    let base = name.to_str().ok_or_else(|| {
        format!(
            "non-utf8 component `{}` of path `{}`",
            name.to_string_lossy(),
            path.display()
        )
    })?;

    let stem = match base.rfind('.') {
        Some(dot) => {
            let (stem, ext) = base.split_at(dot);

            if ext != DOT_EXT {
                None
            } else {
                Some(stem)
            }
        }
        None => None,
    };

    let stem = match stem {
        Some(stem) => stem,
        None => {
            return Err(format!(
                "`{}` is not a package, `{}` does not have .reproto extension",
                path.display(),
                base
            )
            .into());
        }
    };

    let (name_base, version) =
        parse_stem(stem).map_err(|e| format!("bad file: {}: {}", path.display(), e.display()))?;

    Ok((name_base, version))
}

/// Parse a stem into a base name and a version.
pub fn parse_stem<'a>(stem: &'a str) -> Result<(&'a str, Option<Version>)> {
    let mut it = stem.splitn(2, '-');

    if let (Some(name_base), Some(name_version)) = (it.next(), it.next()) {
        let version = Version::parse(name_version).map_err(|e| format!("bad version: {}", e))?;

        return Ok((name_base, Some(version)));
    }

    Ok((stem, None))
}

#[cfg(test)]
mod tests {
    use super::path_to_package;
    use reproto_core::{RpPackage, RpVersionedPackage, Version};

    fn version(version: &str) -> Version {
        Version::parse(version).expect("bad version")
    }

    #[test]
    fn test_path_to_package() {
        let package = RpVersionedPackage::new(
            RpPackage::new(vec!["foo".to_string(), "bar".to_string()]),
            Some(version("2.0.0")),
        );

        assert_eq!(
            package,
            path_to_package("foo/bar-2.0.0.reproto").expect("bad path")
        );

        let package = RpVersionedPackage::new(
            RpPackage::new(vec!["bar".to_string()]),
            Some(version("2.0.0")),
        );

        assert_eq!(
            package,
            path_to_package("bar-2.0.0.reproto").expect("bad path")
        );

        let package = RpVersionedPackage::empty();
        assert_eq!(package, path_to_package(".").expect("bad path"));

        assert!(path_to_package("./foo.txt").is_err());
    }
}
