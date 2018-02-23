//! Compiler for JavaScript Backend

use super::EXT;
use backend::{PackageProcessor, PackageUtils};
use core::{Handle, Loc, RpEnumBody, RpInterfaceBody, RpPackage, RpTupleBody, RpTypeBody,
           RpVersionedPackage};
use core::errors::*;
use js_backend::JsBackend;
use js_file_spec::JsFileSpec;
use trans::Environment;

pub struct JsCompiler<'el> {
    pub handle: &'el Handle,
    pub backend: &'el JsBackend,
}

impl<'el> JsCompiler<'el> {
    pub fn compile(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_files(files)
    }
}

impl<'el> PackageProcessor<'el> for JsCompiler<'el> {
    type Out = JsFileSpec<'el>;

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
}
