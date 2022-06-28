//! # Resolver where we know an exact set of packages to resolve.

use reproto_core::errors::Result;
use reproto_core::{
    Resolved, ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage, RpVersionedPackage, Source,
};
use std::collections::BTreeMap;

pub struct Packages {
    packages: BTreeMap<RpVersionedPackage, Source>,
}

impl Packages {
    pub fn new(packages: BTreeMap<RpVersionedPackage, Source>) -> Self {
        Self { packages }
    }
}

impl Resolver for Packages {
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Option<Resolved>> {
        let mut resolved = Vec::new();

        for (p, source) in &self.packages {
            if p.package != package.package {
                continue;
            }

            match p.version {
                Some(ref version) => {
                    if !package.range.matches(version) {
                        continue;
                    }
                }
                None => {
                    if !package.range.matches_any() {
                        continue;
                    }
                }
            }

            resolved.push(Resolved {
                version: p.version.clone(),
                source: source.clone(),
            });
        }

        Ok(resolved.into_iter().next_back())
    }

    fn resolve_by_prefix(&mut self, package: &RpPackage) -> Result<Vec<ResolvedByPrefix>> {
        let mut out = Vec::new();

        for (p, source) in &self.packages {
            if p.starts_with(package) {
                out.push(ResolvedByPrefix {
                    package: p.clone(),
                    source: source.clone(),
                });
            }
        }

        Ok(out)
    }

    fn resolve_packages(&mut self) -> Result<Vec<ResolvedByPrefix>> {
        self.resolve_by_prefix(&RpPackage::empty())
    }
}
