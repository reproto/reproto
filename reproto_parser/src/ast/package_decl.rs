use std::error::Error;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct PackageDecl {
    pub package: RpPackage,
    pub version: Option<AstLoc<String>>,
}

impl IntoModel for PackageDecl {
    type Output = RpPackageDecl;

    fn into_model(self, path: &Path) -> Result<RpPackageDecl> {
        let version = if let Some(version) = self.version.into_model(path)? {
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
