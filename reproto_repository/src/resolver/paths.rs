use std::path::PathBuf;
use super::*;

const EXT: &str = "reproto";

pub struct Paths {
    paths: Vec<PathBuf>,
}

impl Paths {
    pub fn new(paths: Vec<PathBuf>) -> Paths {
        Paths { paths: paths }
    }
}

impl Resolver for Paths {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<PathBuf>> {
        let candidates: Vec<_> = self.paths
            .iter()
            .map(|p| {
                let mut path: PathBuf = (*p).to_owned();

                for part in &package.package.parts {
                    path = path.join(part);
                }

                path.set_extension(EXT);
                path
            })
            .collect();

        let files = candidates.iter().filter(|p| p.is_file()).map(Clone::clone).collect();
        Ok(files)
    }
}
