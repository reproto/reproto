use std::error::Error;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct PackageDecl<'input> {
    pub package: RpPackage,
    pub version: Option<AstLoc<'input, String>>,
}

impl<'input> IntoModel for PackageDecl<'input> {
    type Output = RpPackageDecl;

    fn into_model(self) -> Result<RpPackageDecl> {
        let version = if let Some(version) = self.version.into_model()? {
            let (version, pos) = version.both();

            match Version::parse(&version) {
                Ok(version) => Some(RpLoc::new(version, pos)),
                Err(e) => return Err(ErrorKind::Pos(e.description().to_owned(), pos).into()),
            }
        } else {
            None
        };

        Ok(RpPackageDecl {
            package: self.package,
            version: version,
        })
    }
}
