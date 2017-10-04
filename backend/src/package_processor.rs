use super::into_bytes::IntoBytes;
use core::{Loc, RpDecl, RpEnumBody, RpInterfaceBody, RpName, RpPackage, RpServiceBody,
           RpTupleBody, RpTypeBody, RpVersionedPackage};
use environment::Environment;
use errors::*;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub trait PackageProcessor<'el>
where
    Self: 'el + Sized,
{
    type Out: Default + IntoBytes<Self>;

    fn ext(&self) -> &str;

    fn env(&self) -> &'el Environment;

    fn out_path(&self) -> &Path;

    fn default_process(&self, _: &mut Self::Out, name: &'el RpName) -> Result<()> {
        warn!("not supported: {}", name);
        Ok(())
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage;

    fn process_interface(
        &self,
        out: &mut Self::Out,
        body: &'el Loc<RpInterfaceBody>,
    ) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el Loc<RpTypeBody>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el Loc<RpTupleBody>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el Loc<RpEnumBody>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el Loc<RpServiceBody>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn populate_files(&self) -> Result<BTreeMap<RpVersionedPackage, Self::Out>> {
        self.do_populate_files(|_| Ok(()))
    }

    fn do_populate_files<F>(
        &self,
        mut callback: F,
    ) -> Result<BTreeMap<RpVersionedPackage, Self::Out>>
    where
        F: FnMut(&'el Loc<RpDecl>) -> Result<()>,
    {
        use self::RpDecl::*;

        let mut files = BTreeMap::new();

        // Process all types discovered so far.
        self.env().for_each_decl(|decl| {
            callback(decl)?;

            let mut out = files.entry(decl.name().package.clone()).or_insert_with(
                Self::Out::default,
            );

            match *decl.value() {
                Interface(ref b) => self.process_interface(&mut out, b),
                Type(ref b) => self.process_type(&mut out, b),
                Tuple(ref b) => self.process_tuple(&mut out, b),
                Enum(ref b) => self.process_enum(&mut out, b),
                Service(ref b) => self.process_service(&mut out, b),
            }
        })?;

        Ok(files)
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<PathBuf> {
        let full_path = self.out_path().to_owned();
        let mut full_path = package.parts.iter().fold(full_path, |a, b| a.join(b));
        full_path.set_extension(self.ext());
        Ok(full_path)
    }

    fn setup_module_path(&self, package: &RpPackage) -> Result<PathBuf> {
        let full_path = self.resolve_full_path(package)?;

        if let Some(parent) = full_path.parent() {
            if !parent.is_dir() {
                debug!("+dir: {}", parent.display());
                fs::create_dir_all(parent)?;
            }
        }

        Ok(full_path)
    }

    fn write_files(&'el self, files: BTreeMap<RpVersionedPackage, Self::Out>) -> Result<()> {
        for (package, out) in files {
            let package = self.processed_package(&package);
            let full_path = self.setup_module_path(&package)?;

            debug!("+module: {}", full_path.display());

            let mut f = File::create(full_path)?;
            let bytes = out.into_bytes(self)?;
            f.write_all(&bytes)?;
            f.flush()?;
        }

        Ok(())
    }
}
