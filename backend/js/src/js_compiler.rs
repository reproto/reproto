//! Compiler for JavaScript Backend

use super::EXT;
use backend::{Environment, PackageProcessor, PackageUtils};
use backend::errors::*;
use core::{Loc, RpEnumBody, RpInterfaceBody, RpPackage, RpTupleBody, RpTypeBody,
           RpVersionedPackage};
use js_backend::JsBackend;
use js_file_spec::JsFileSpec;
use std::path::{Path, PathBuf};

pub struct JsCompiler<'el> {
    pub out_path: PathBuf,
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
}
