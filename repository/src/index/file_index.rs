use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use super::*;
use url_serde;

const CONFIG_JSON: &'static str = "config.json";
const METADATA_JSON: &'static str = "metadata.json";

#[derive(Deserialize)]
pub struct Config {
    #[serde(with="url_serde")]
    objects: Url,
}

pub struct FileIndex {
    path: PathBuf,
}

impl FileIndex {
    pub fn new<P: AsRef<Path> + ?Sized>(path: &P) -> FileIndex {
        FileIndex { path: path.as_ref().to_owned() }
    }

    fn path_for(&self, package: &RpPackage) -> PathBuf {
        package.parts.iter().fold(self.path.clone(), |path, next| path.join(next))
    }
}

impl Index for FileIndex {
    fn resolve(&self, package: &RpPackage, version_req: &VersionReq) -> Result<Vec<Deployment>> {
        let path = self.path_for(package).join(METADATA_JSON);

        if !path.is_file() {
            return Ok(vec![]);
        }

        let f = File::open(&path)?;
        let mut reader = BufReader::new(f);

        let mut out = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;

            let deployment: Deployment = serde_json::from_str(&line).map_err(|e| {
                    format!("{}: bad deployment on line #{}: {}",
                            path.display(),
                            i + 1,
                            e)
                })?;

            if version_req.matches(&deployment.version) {
                out.push(deployment);
            }
        }

        Ok(out)
    }

    fn objects_url(&self) -> Result<Url> {
        let config = self.path.join(CONFIG_JSON);

        let mut f = File::open(&config)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let config: Config = serde_json::from_str(content.as_str())
            .map_err(|e| format!("{}: bad config file: {}", config.display(), e))?;

        Ok(config.objects)
    }
}
