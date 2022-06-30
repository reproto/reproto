use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use reproto_core::errors::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Address to listen to.
    #[serde(default = "default_listen_address")]
    pub listen_address: String,
    /// Objects path.
    #[serde(default = "default_objects")]
    pub objects: PathBuf,
    /// Index path.
    #[serde(default = "default_index")]
    pub index: PathBuf,
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

fn default_index() -> PathBuf {
    Path::new("./index").to_owned()
}

fn default_max_file_size() -> u64 {
    1_000_000u64
}

impl Default for Config {
    fn default() -> Config {
        Config {
            listen_address: default_listen_address(),
            objects: default_objects(),
            index: default_index(),
            max_file_size: default_max_file_size(),
        }
    }
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path = path.as_ref();
    let mut f = File::open(path)
        .map_err(|e| format!("failed to open config: {}: {}", path.display(), e))?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let config: Config = toml::from_str(content.as_str())
        .map_err(|e| format!("{}: bad config: {}", path.display(), e))?;

    Ok(config)
}
