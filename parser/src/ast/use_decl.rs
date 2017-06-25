use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct UseDecl<'input> {
    pub package: RpLoc<RpPackage>,
    pub version_req: Option<RpLoc<VersionReq>>,
    pub alias: Option<&'input str>,
}

impl<'input> IntoModel for UseDecl<'input> {
    type Output = RpUseDecl;

    fn into_model(self) -> Result<RpUseDecl> {
        let use_decl = RpUseDecl {
            package: self.package.into_model()?,
            version_req: self.version_req,
            alias: self.alias.map(ToOwned::to_owned),
        };

        Ok(use_decl)
    }
}
