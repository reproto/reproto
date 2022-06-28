//! Encapsulate home environment

use crate::config::read_config;
use dirs;
use reproto_core::errors::Result;
use std::path::PathBuf;

pub struct ConfigEnvironment {
    pub config: PathBuf,
    pub repo_dir: PathBuf,
    pub cache_home: PathBuf,
    pub releases_dir: PathBuf,
    pub bin_home: PathBuf,
    pub index: Option<String>,
    pub objects: Option<String>,
}

impl ConfigEnvironment {
    pub fn new() -> Result<Option<ConfigEnvironment>> {
        let config_dir = match dirs::config_dir() {
            None => return Ok(None),
            Some(config_dir) => config_dir,
        };

        let mut cache_home = match dirs::cache_dir() {
            None => return Ok(None),
            Some(cache_home) => cache_home,
        };

        let data_home = match dirs::data_dir() {
            None => return Ok(None),
            Some(data_home) => data_home,
        };

        let bin_home = match dirs::executable_dir() {
            None => return Ok(None),
            Some(bin_home) => bin_home,
        };

        let config = config_dir.join("config.toml");
        let mut repo_dir = config_dir.join("git");

        let mut index = None;
        let mut objects = None;

        if config.is_file() {
            let config = read_config(&config)?;

            if let Some(repository) = config.repository {
                // set values from configuration (if not already set).
                index = index.or(repository.index);
                objects = objects.or(repository.objects);
            }

            if let Some(out) = config.cache_home {
                cache_home = out;
            }

            if let Some(out) = config.repo_dir {
                repo_dir = out;
            }
        }

        let releases_dir = data_home.join("releases");

        return Ok(Some(ConfigEnvironment {
            config,
            repo_dir,
            cache_home,
            releases_dir,
            bin_home,
            index,
            objects,
        }));
    }
}
