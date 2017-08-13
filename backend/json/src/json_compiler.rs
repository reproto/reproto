use super::*;
use serde_json;
use std::fmt::Write as FmtWrite;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub struct JsonCompiler<'a> {
    pub out_path: PathBuf,
    pub processor: &'a JsonBackend,
}

impl<'a> JsonCompiler<'a> {
    pub fn compile(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_files(files)?;
        Ok(())
    }
}

impl<'a> PackageProcessor<'a> for JsonCompiler<'a> {
    type Out = Collector;

    fn ext(&self) -> &str {
        EXT
    }

    fn env(&self) -> &Environment {
        &self.processor.env
    }

    fn out_path(&self) -> &Path {
        &self.out_path
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.processor.package(package)
    }

    fn default_process(&self, _: &mut Self::Out, _: &RpTypeId, _: &Pos) -> Result<()> {
        Ok(())
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<PathBuf> {
        let mut full_path = self.out_path().join(self.processor.package_file(package));
        full_path.set_extension(self.ext());
        Ok(full_path)
    }

    fn process_service(
        &self,
        out: &mut Self::Out,
        _: &RpTypeId,
        _: &Pos,
        body: Rc<RpServiceBody>,
    ) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(&body)?)?;
        Ok(())
    }

    fn process_enum(
        &self,
        out: &mut Self::Out,
        _: &RpTypeId,
        _: &Pos,
        body: Rc<RpEnumBody>,
    ) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(&body)?)?;
        Ok(())
    }

    fn process_interface(
        &self,
        out: &mut Self::Out,
        _: &RpTypeId,
        _: &Pos,
        body: Rc<RpInterfaceBody>,
    ) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(&body)?)?;
        Ok(())
    }

    fn process_type(
        &self,
        out: &mut Self::Out,
        _: &RpTypeId,
        _: &Pos,
        body: Rc<RpTypeBody>,
    ) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(&body)?)?;
        Ok(())
    }

    fn process_tuple(
        &self,
        out: &mut Self::Out,
        _: &RpTypeId,
        _: &Pos,
        body: Rc<RpTupleBody>,
    ) -> Result<()> {
        writeln!(out, "{}", serde_json::to_string(&body)?)?;
        Ok(())
    }
}
