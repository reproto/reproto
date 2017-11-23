//! Encapsulate home environment

use config::read_config;
use errors::*;
use std::env;
use std::path::PathBuf;

pub struct ConfigEnv {
    pub config: PathBuf,
    pub repo_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub index: Option<String>,
    pub objects: Option<String>,
}

impl ConfigEnv {
    pub fn new() -> Result<Option<ConfigEnv>> {
        if let Some(home_dir) = env::home_dir() {
            let reproto_dir = home_dir.join(".reproto");
            let config = reproto_dir.join("config.toml");

            let mut repo_dir = reproto_dir.join("git");
            let mut cache_dir = reproto_dir.join("cache");
            let mut index = None;
            let mut objects = None;

            if config.is_file() {
                let config = read_config(&config)?;

                if let Some(repository) = config.repository {
                    // set values from configuration (if not already set).
                    index = index.or(repository.index);
                    objects = objects.or(repository.objects);
                }

                if let Some(out) = config.cache_dir {
                    cache_dir = out;
                }

                if let Some(out) = config.repo_dir {
                    repo_dir = out;
                }
            }

            return Ok(Some(ConfigEnv {
                config: config,
                repo_dir: repo_dir,
                cache_dir: cache_dir,
                index: index,
                objects: objects,
            }));
        }

        Ok(None)
    }
}
