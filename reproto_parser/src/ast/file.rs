use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct File<'input> {
    pub package_decl: AstLoc<PackageDecl>,
    pub options: Vec<AstLoc<OptionDecl<'input>>>,
    pub uses: Vec<AstLoc<UseDecl>>,
    pub decls: Vec<AstLoc<Decl<'input>>>,
}

impl<'input> IntoModel for File<'input> {
    type Output = RpFile;

    fn into_model(self, path: &Path) -> Result<RpFile> {
        let options = Options::new(self.options.clone().into_model(path)?);

        let file = RpFile {
            package_decl: self.package_decl.into_model(path)?,
            options: options,
            uses: self.uses.into_model(path)?,
            decls: self.decls.into_model(path)?,
        };

        Ok(file)
    }
}
