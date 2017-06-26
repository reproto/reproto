use serde_json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
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

    /// # read package metadata
    ///
    /// returns a tuple where the first is all matching deployments, and the second is a boolean
    /// indicating if some deployments have been omitted or not.
    fn read_package<F>(&self, package: &RpPackage, filter: F) -> Result<(Vec<Deployment>, bool)>
        where F: Fn(&Deployment) -> bool
    {
        let path = self.path_for(package).join(METADATA_JSON);

        if !path.is_file() {
            return Ok((vec![], false));
        }

        let f = File::open(&path)?;
        let reader = BufReader::new(f);
        let mut out = Vec::new();
        let mut non_match = false;

        for (i, line) in reader.lines().enumerate() {
            let line = line?;

            let deployment: Deployment = serde_json::from_str(&line).map_err(|e| {
                    format!("{}: bad deployment on line #{}: {}",
                            path.display(),
                            i + 1,
                            e)
                })?;

            if filter(&deployment) {
                out.push(deployment);
            } else {
                non_match = true;
            }
        }

        Ok((out, non_match))
    }

    fn write_package<I>(&self, package: &RpPackage, deployments: I) -> Result<()>
        where I: IntoIterator<Item = Deployment>
    {
        let target = self.path_for(package).join(METADATA_JSON);
        debug!("writing: {}", target.display());

        let mut tmp_target = target.clone();
        tmp_target.set_extension(".tmp");

        if let Some(parent) = tmp_target.parent() {
            if !parent.is_dir() {
                debug!("creating directory: {}", parent.display());
                fs::create_dir_all(parent)?;
            }
        }

        {
            let mut f = File::create(&tmp_target)?;
            let it = deployments.into_iter();

            for deployment in it {
                writeln!(f, "{}", serde_json::to_string(&deployment)?)?;
            }
        }

        fs::rename(&tmp_target, &target)?;
        Ok(())
    }

    fn path_for(&self, package: &RpPackage) -> PathBuf {
        package.parts.iter().fold(self.path.clone(), |path, next| path.join(next))
    }
}

impl Index for FileIndex {
    fn resolve(&self,
               package: &RpPackage,
               version_req: Option<&VersionReq>)
               -> Result<Vec<Deployment>> {
        self.read_package(package,
                          |d| version_req.map(|v| v.matches(&d.version)).unwrap_or(true))
            .map(|r| r.0)
    }

    fn put_version(&self,
                   checksum: &Checksum,
                   package: &RpPackage,
                   version: &Version)
                   -> Result<()> {
        let (mut deployments, other_match) = self.read_package(package, |d| d.version != *version)?;

        if other_match {
            return Err(format!("{}@{}: already published", package, version).into());
        }

        deployments.push(Deployment::new(version.clone(), checksum.clone()));
        deployments.sort_by(|a, b| a.version.cmp(&b.version));
        self.write_package(package, deployments)?;
        Ok(())
    }

    fn get_deployments(&self, package: &RpPackage, version: &Version) -> Result<Vec<Deployment>> {
        self.read_package(package, |d| d.version == *version).map(|r| r.0)
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
