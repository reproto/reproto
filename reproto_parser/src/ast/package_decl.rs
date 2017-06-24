use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct PackageDecl {
    pub package: RpPackage,
}

impl IntoModel for PackageDecl {
    type Output = RpPackageDecl;

    fn into_model(self) -> Result<RpPackageDecl> {
        Ok(RpPackageDecl { package: self.package })
    }
}
