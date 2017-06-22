use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct File<'input> {
    pub package_decl: RpLoc<PackageDecl>,
    pub options: Vec<RpLoc<OptionDecl<'input>>>,
    pub uses: Vec<RpLoc<UseDecl<'input>>>,
    pub decls: Vec<RpLoc<Decl<'input>>>,
}

impl<'input> IntoModel for File<'input> {
    type Output = RpFile;

    fn into_model(self) -> Result<RpFile> {
        let options = Options::new(self.options.into_model()?);

        let file = RpFile {
            package_decl: self.package_decl.into_model()?,
            options: options,
            uses: self.uses.into_model()?,
            decls: self.decls.into_model()?,
        };

        Ok(file)
    }
}
