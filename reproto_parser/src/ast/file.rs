use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct File<'input> {
    pub package_decl: AstLoc<'input, PackageDecl<'input>>,
    pub options: Vec<AstLoc<'input, OptionDecl<'input>>>,
    pub uses: Vec<AstLoc<'input, UseDecl<'input>>>,
    pub decls: Vec<AstLoc<'input, Decl<'input>>>,
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
