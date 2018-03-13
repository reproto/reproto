use super::{EXT, INIT_PY};
use backend::{PackageProcessor, PackageUtils};
use core::{Handle, RelativePathBuf, RpDecl, RpEnumBody, RpInterfaceBody, RpPackage, RpServiceBody,
           RpTupleBody, RpTypeBody, RpVersionedPackage};
use core::errors::*;
use python_backend::PythonBackend;
use python_file_spec::PythonFileSpec;
use std::collections::BTreeMap;
use trans::Environment;

pub struct PythonCompiler<'el> {
    pub handle: &'el Handle,
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

    fn handle(&self) -> &'el Handle {
        self.handle
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.backend.package(package)
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody) -> Result<()> {
        self.backend.process_tuple(out, body)
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        self.backend.process_enum(out, body)
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
        self.backend.process_type(out, body)
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody) -> Result<()> {
        self.backend.process_interface(out, body)
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el RpServiceBody) -> Result<()> {
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

    fn resolve_full_path(&self, package: &RpPackage) -> Result<RelativePathBuf> {
        let handle = self.handle();

        let mut full_path = RelativePathBuf::new();
        let mut iter = package.parts.iter().peekable();

        while let Some(part) = iter.next() {
            full_path = full_path.join(part);

            if iter.peek().is_none() {
                continue;
            }

            if !handle.is_dir(&full_path) {
                debug!("+dir: {}", full_path.display());
                handle.create_dir_all(&full_path)?;
            }

            let init_path = full_path.join(INIT_PY);

            if !handle.is_file(&init_path) {
                debug!("+init: {}", init_path.display());
                handle.create(&init_path)?;
            }
        }

        full_path.set_extension(self.ext());
        Ok(full_path)
    }
}
