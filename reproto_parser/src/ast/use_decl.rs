use std::error::Error;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct UseDecl {
    pub package: AstLoc<RpPackage>,
    pub version_req: Option<AstLoc<String>>,
    pub alias: Option<String>,
}

impl IntoModel for UseDecl {
    type Output = RpUseDecl;

    fn into_model(self, path: &Path) -> Result<RpUseDecl> {
        let version_req = if let Some(version_req) = self.version_req.into_model(path)? {
            let (version_req, pos) = version_req.both();

            match VersionReq::parse(&version_req) {
                Ok(version_req) => Some(RpLoc::new(version_req, pos)),
                Err(e) => return Err(ErrorKind::Pos(e.description().to_owned(), pos).into()),
            }
        } else {
            None
        };

        let use_decl = RpUseDecl {
            package: self.package.into_model(path)?,
            version_req: version_req,
            alias: self.alias,
        };

        Ok(use_decl)
    }
}
