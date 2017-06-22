use std::error::Error;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct UseDecl<'input> {
    pub package: RpLoc<RpPackage>,
    pub version_req: Option<RpLoc<String>>,
    pub alias: Option<&'input str>,
}

impl<'input> IntoModel for UseDecl<'input> {
    type Output = RpUseDecl;

    fn into_model(self) -> Result<RpUseDecl> {
        let version_req = if let Some(version_req) = self.version_req.into_model()? {
            let (version_req, pos) = version_req.both();

            match VersionReq::parse(&version_req) {
                Ok(version_req) => Some(RpLoc::new(version_req, pos)),
                Err(e) => return Err(ErrorKind::Pos(e.description().to_owned(), pos.into()).into()),
            }
        } else {
            None
        };

        let use_decl = RpUseDecl {
            package: self.package.into_model()?,
            version_req: version_req,
            alias: self.alias.map(ToOwned::to_owned),
        };

        Ok(use_decl)
    }
}
