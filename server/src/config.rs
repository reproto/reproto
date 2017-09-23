use super::errors::*;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Address to listen to.
    #[serde(default = "default_listen_address")]
    pub listen_address: String,
    /// Objects path.
    #[serde(default = "default_objects")]
    pub objects: PathBuf,
    /// Max file size permitted during upload.
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
}

fn default_listen_address() -> String {
    "127.0.0.1:1234".to_owned()
}

fn default_objects() -> PathBuf {
    Path::new("./objects").to_owned()
}

fn default_max_file_size() -> u64 {
    10000000u64
}

impl Default for Config {
    fn default() -> Config {
        Config {
            listen_address: default_listen_address(),
            objects: default_objects(),
            max_file_size: default_max_file_size(),
        }
    }
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path = path.as_ref();
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let config: Config = toml::from_str(content.as_str()).map_err(|e| {
        format!("{}: bad config: {}", path.display(), e)
    })?;

    Ok(config)
}
