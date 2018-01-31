extern crate futures_cpupool;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate reproto_repository;
extern crate reproto_server;

use futures_cpupool::CpuPool;
use hyper::server::Http;
use reproto_repository::objects_from_path;
use reproto_server::errors::*;
use reproto_server::reproto_service;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Get the configuration path to load.
fn config_path() -> Result<Option<PathBuf>> {
    use self::env::VarError;

    match env::var("REPROTO_SERVER_CONFIG") {
        Ok(path) => Ok(Some(Path::new(path.as_str()).to_owned())),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => return Err(e.into()),
    }
}

fn entry() -> Result<()> {
    pretty_env_logger::init()?;

    let config = if let Some(path) = config_path()? {
        reproto_server::config::read_config(path)?
    } else {
        reproto_server::config::Config::default()
    };

    let listen_address = config.listen_address.parse()?;
    let objects = config.objects;
    let max_file_size = config.max_file_size;

    let pool = Arc::new(CpuPool::new_num_cpus());
    let setup_pool = pool.clone();
    let objects = Arc::new(Mutex::new(objects_from_path(objects)?));

    let setup = move || {
        Ok(reproto_service::ReprotoService {
            max_file_size: max_file_size,
            pool: setup_pool.clone(),
            objects: objects.clone(),
        })
    };

    let server = Http::new().bind(&listen_address, setup)?;

    info!("Listening on http://{}", server.local_addr()?);
    server.run()?;
    Ok(())
}

fn main() {
    if let Err(e) = entry() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("{:?}", backtrace);
        }
    }

    info!("Shutting down");
}
