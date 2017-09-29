use collecting::Collecting;
use core::{Loc, RpDecl, RpEnumBody, RpInterfaceBody, RpName, RpPackage, RpServiceBody,
           RpTupleBody, RpTypeBody, RpVersionedPackage, WithPos};
use environment::Environment;
use errors::*;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub trait PackageProcessor<'a>
where
    Self: 'a + Sized,
{
    type Out: Collecting<'a, Processor = Self>;

    fn ext(&self) -> &str;

    fn env(&self) -> &Environment;

    fn out_path(&self) -> &Path;

    fn default_process(&self, _: &mut Self::Out, name: &RpName) -> Result<()> {
        warn!("not supported: {}", name);
        Ok(())
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage;

    fn process_interface(
        &self,
        out: &mut Self::Out,
        name: &RpName,
        _: Rc<Loc<RpInterfaceBody>>,
    ) -> Result<()> {
        self.default_process(out, name)
    }

    fn process_type(
        &self,
        out: &mut Self::Out,
        name: &RpName,
        _: Rc<Loc<RpTypeBody>>,
    ) -> Result<()> {
        self.default_process(out, name)
    }

    fn process_tuple(
        &self,
        out: &mut Self::Out,
        name: &RpName,
        _: Rc<Loc<RpTupleBody>>,
    ) -> Result<()> {
        self.default_process(out, name)
    }

    fn process_enum(
        &self,
        out: &mut Self::Out,
        name: &RpName,
        _: Rc<Loc<RpEnumBody>>,
    ) -> Result<()> {
        self.default_process(out, name)
    }

    fn process_service(
        &self,
        out: &mut Self::Out,
        name: &RpName,
        _: Rc<Loc<RpServiceBody>>,
    ) -> Result<()> {
        self.default_process(out, name)
    }

    fn populate_files(&self) -> Result<BTreeMap<RpVersionedPackage, Self::Out>> {
        self.do_populate_files(|_, _| Ok(()))
    }

    fn do_populate_files<F>(
        &self,
        mut callback: F,
    ) -> Result<BTreeMap<RpVersionedPackage, Self::Out>>
    where
        F: FnMut(Rc<RpName>, Rc<Loc<RpDecl>>) -> Result<()>,
    {
        use self::RpDecl::*;

        let mut files = BTreeMap::new();

        // Process all types discovered so far.
        self.env().for_each_decl(|name, decl| {
            callback(name.clone(), decl.clone())?;

            let mut out = files.entry(name.package.clone()).or_insert_with(
                Self::Out::new,
            );

            match **decl {
                Interface(ref b) => {
                    self.process_interface(&mut out, name.as_ref(), b.clone())
                        .with_pos(b.pos())
                }
                Type(ref b) => {
                    self.process_type(&mut out, name.as_ref(), b.clone())
                        .with_pos(b.pos())
                }
                Tuple(ref b) => {
                    self.process_tuple(&mut out, name.as_ref(), b.clone())
                        .with_pos(b.pos())
                }
                Enum(ref b) => {
                    self.process_enum(&mut out, name.as_ref(), b.clone())
                        .with_pos(b.pos())
                }
                Service(ref b) => {
                    self.process_service(&mut out, name.as_ref(), b.clone())
                        .with_pos(b.pos())
                }
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

    fn write_files(&self, files: BTreeMap<RpVersionedPackage, Self::Out>) -> Result<()> {
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
