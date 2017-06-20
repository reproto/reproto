use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct File {
    pub package_decl: AstLoc<PackageDecl>,
    pub options: Vec<AstLoc<OptionDecl>>,
    pub uses: Vec<AstLoc<UseDecl>>,
    pub decls: Vec<AstLoc<Decl>>,
}

impl IntoModel for File {
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
