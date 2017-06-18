use std::fs::File;
use std::io::Read;
use super::*;
use toml;

const EXT: &'static str = "reproto";
const METADATA: &'static str = "metadata";

pub struct Filesystem {
    path: PathBuf,
}

impl Filesystem {
    pub fn new(path: PathBuf) -> Filesystem {
        Filesystem { path: path }
    }
}

impl Resolver for Filesystem {
    fn resolve(&self, package: &RpRequiredPackage) -> Result<Vec<PathBuf>> {
        let path = package.package.parts.iter().fold(self.path.clone(), |a, b| a.join(b));
        let metadata_path = path.join(METADATA);

        if !metadata_path.is_file() {
            return Ok(vec![]);
        }

        let mut f = File::open(&metadata_path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let metadata: Metadata = toml::from_str(&s)?;

        let versions = if let Some(ref version_req) = package.version_req {
            metadata.versions
                .into_iter()
                .filter(|version| version_req.matches(version))
                .collect()
        } else {
            metadata.versions
        };

        let mut out = Vec::new();

        for version in versions {
            let path = path.join(format!("{}.{}", version, EXT));

            if !path.is_file() {
                continue;
            }

            out.push(path);
        }

        Ok(out)
    }
}
