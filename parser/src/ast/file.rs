use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct File<'input> {
    pub options: Vec<Loc<OptionDecl<'input>>>,
    pub uses: Vec<Loc<UseDecl<'input>>>,
    pub decls: Vec<Loc<Decl<'input>>>,
}

impl<'input> IntoModel for File<'input> {
    type Output = RpFile;

    fn into_model(self) -> Result<RpFile> {
        let options = Options::new(self.options.into_model()?);

        let file = RpFile {
            options: options,
            uses: self.uses.into_model()?,
            decls: self.decls.into_model()?,
        };

        Ok(file)
    }
}
