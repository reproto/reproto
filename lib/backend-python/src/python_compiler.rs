use super::{EXT, INIT_PY};
use backend::{Environment, PackageProcessor, PackageUtils};
use backend::errors::*;
use core::{Loc, RpDecl, RpEnumBody, RpInterfaceBody, RpPackage, RpServiceBody, RpTupleBody,
           RpTypeBody, RpVersionedPackage};
use python_backend::PythonBackend;
use python_file_spec::PythonFileSpec;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub struct PythonCompiler<'el> {
    pub out_path: PathBuf,
    pub backend: &'el PythonBackend,
}

impl<'el> PythonCompiler<'el> {
    pub fn compile(&self) -> Result<()> {
        self.write_files(self.populate_files()?)
    }
}

impl<'el> PackageProcessor<'el> for PythonCompiler<'el> {
    type Out = PythonFileSpec<'el>;

    fn ext(&self) -> &str {
        EXT
    }

    fn env(&self) -> &'el Environment {
        &self.backend.env
    }

    fn out_path(&self) -> &Path {
        &self.out_path
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.backend.package(package)
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el Loc<RpTupleBody>) -> Result<()> {
        self.backend.process_tuple(out, body)
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el Loc<RpEnumBody>) -> Result<()> {
        self.backend.process_enum(out, body)
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el Loc<RpTypeBody>) -> Result<()> {
        self.backend.process_type(out, body)
    }

    fn process_interface(
        &self,
        out: &mut Self::Out,
        body: &'el Loc<RpInterfaceBody>,
    ) -> Result<()> {
        self.backend.process_interface(out, body)
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el Loc<RpServiceBody>) -> Result<()> {
        self.backend.process_service(out, body)
    }

    fn populate_files(&self) -> Result<BTreeMap<RpVersionedPackage, PythonFileSpec<'el>>> {
        let mut enums = Vec::new();

        let mut files = self.do_populate_files(|decl| {
            if let RpDecl::Enum(ref body) = *decl {
                enums.push(body);
            }

            Ok(())
        })?;

        // Process picked up enums.
        // These are added to the end of the file to declare enums:
        // https://docs.python.org/3/library/enum.html
        for body in enums {
            if let Some(ref mut file_spec) = files.get_mut(&body.name.package) {
                file_spec.0.push(self.backend.enum_variants(&body)?);
            } else {
                return Err(format!("missing file for package: {}", &body.name.package).into());
            }
        }

        Ok(files)
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<PathBuf> {
        let mut full_path = self.out_path().to_owned();
        let mut iter = package.parts.iter().peekable();

        while let Some(part) = iter.next() {
            full_path = full_path.join(part);

            if iter.peek().is_none() {
                continue;
            }

            if !full_path.is_dir() {
                debug!("+dir: {}", full_path.display());
                fs::create_dir_all(&full_path)?;
            }

            let init_path = full_path.join(INIT_PY);

            if !init_path.is_file() {
                debug!("+init: {}", init_path.display());
                File::create(init_path)?;
            }
        }

        full_path.set_extension(self.ext());
        Ok(full_path)
    }
}
