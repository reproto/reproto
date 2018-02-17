//! Compiler for JSON

use super::EXT;
use backend::{Environment, PackageProcessor, PackageUtils};
use collector::Collector;
use core::{Loc, RpEnumBody, RpInterfaceBody, RpName, RpPackage, RpServiceBody, RpTupleBody,
           RpTypeBody, RpVersionedPackage};
use core::errors::*;
use json_backend::JsonBackend;
use serde_json;
use std::fmt::Write;
use std::path::{Path, PathBuf};

pub struct JsonCompiler<'el> {
    pub out_path: PathBuf,
    pub processor: &'el JsonBackend,
}

impl<'el> JsonCompiler<'el> {
    pub fn compile(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_files(files)?;
        Ok(())
    }
}

impl<'el> PackageProcessor<'el> for JsonCompiler<'el> {
    type Out = Collector;

    fn ext(&self) -> &str {
        EXT
    }

    fn env(&self) -> &'el Environment {
        &self.processor.env
    }

    fn out_path(&self) -> &Path {
        &self.out_path
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.processor.package(package)
    }

    fn default_process(&self, _: &mut Self::Out, _: &RpName) -> Result<()> {
        Ok(())
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<PathBuf> {
        let mut full_path = self.out_path().join(self.processor.package_file(package));
        full_path.set_extension(self.ext());
        Ok(full_path)
    }

    fn process_service(&self, out: &mut Self::Out, body: &Loc<RpServiceBody>) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(body)?)?;
        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el Loc<RpEnumBody>) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(body)?)?;
        Ok(())
    }

    fn process_interface(
        &self,
        out: &mut Self::Out,
        body: &'el Loc<RpInterfaceBody>,
    ) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(body)?)?;
        Ok(())
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el Loc<RpTypeBody>) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(body)?)?;
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el Loc<RpTupleBody>) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(body)?)?;
        Ok(())
    }
}
