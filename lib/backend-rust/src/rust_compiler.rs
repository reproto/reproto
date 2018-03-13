//! Compiler for Rust Backend

use super::{EXT, MOD};
use backend::{PackageProcessor, PackageUtils};
use core::{Handle, RelativePath, RelativePathBuf, RpEnumBody, RpInterfaceBody, RpName, RpPackage,
           RpServiceBody, RpTupleBody, RpTypeBody, RpVersionedPackage};
use core::errors::*;
use rust_backend::RustBackend;
use rust_file_spec::RustFileSpec;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use trans::Environment;

pub struct RustCompiler<'a> {
    pub handle: &'a Handle,
    pub backend: &'a RustBackend,
}

impl<'a> RustCompiler<'a> {
    pub fn compile(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_mod_files(&files)?;
        self.write_files(files)
    }

    fn write_mod_files(&self, files: &BTreeMap<RpVersionedPackage, RustFileSpec>) -> Result<()> {
        let mut packages: BTreeMap<RelativePathBuf, BTreeSet<String>> = BTreeMap::new();
        let mut root_names = BTreeSet::new();

        for (key, _) in files {
            let mut current = RelativePathBuf::new();

            let mut it = self.backend.package(key).parts.into_iter().peekable();

            if let Some(root) = it.peek() {
                root_names.insert(root.to_owned());
            }

            while let Some(part) = it.next() {
                current = current.join(part);

                if let Some(next) = it.peek() {
                    let mut full_path = current.join(MOD);
                    full_path.set_extension(self.ext());

                    packages
                        .entry(full_path)
                        .or_insert_with(BTreeSet::new)
                        .insert(next.clone());
                }
            }
        }

        let mut root_mod = RelativePathBuf::new().join(MOD);
        root_mod.set_extension(self.ext());
        packages.insert(root_mod, root_names);

        let handle = self.handle();

        for (full_path, children) in packages {
            let parent = full_path.parent().unwrap_or(RelativePath::new("."));

            if !self.handle.is_dir(&parent) {
                debug!("+dir: {}", parent.display());
                handle.create_dir_all(&parent)?;
            }

            if !handle.is_file(&full_path) {
                debug!("+mod: {}", full_path.display());
                let mut f = handle.create(&full_path)?;

                for child in children {
                    writeln!(f, "pub mod {};", child)?;
                }
            }
        }

        Ok(())
    }
}

impl<'p> PackageProcessor<'p> for RustCompiler<'p> {
    type Out = RustFileSpec<'p>;

    fn ext(&self) -> &str {
        EXT
    }

    fn env(&self) -> &'p Environment {
        &self.backend.env
    }

    fn handle(&self) -> &'p Handle {
        self.handle
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.backend.package(package)
    }

    fn default_process(&self, _out: &mut Self::Out, _: &RpName) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'p RpTupleBody) -> Result<()> {
        self.backend.process_tuple(out, body)?;
        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'p RpEnumBody) -> Result<()> {
        self.backend.process_enum(out, body)
    }

    fn process_type(&self, out: &mut Self::Out, body: &'p RpTypeBody) -> Result<()> {
        self.backend.process_type(out, body)
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'p RpInterfaceBody) -> Result<()> {
        self.backend.process_interface(out, body)
    }

    fn process_service(&self, out: &mut Self::Out, body: &'p RpServiceBody) -> Result<()> {
        self.backend.process_service(out, body)
    }
}
