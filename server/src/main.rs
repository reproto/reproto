extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate reproto_core as core;
extern crate reproto_repository as repository;
extern crate reproto_server as server;

use core::errors::Result;
use futures_cpupool::CpuPool;
use hyper::rt::{self, Future};
use hyper::server::Server;
use repository::{index_from_path, objects_from_path};
use server::reproto_service;
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
    pretty_env_logger::init();

    let config = if let Some(path) = config_path()? {
        server::config::read_config(path)?
    } else {
        server::config::Config::default()
    };

    let server::config::Config {listen_address, objects, index, max_file_size} = config;
    let listen_address = listen_address.parse()?;

    let pool = Arc::new(CpuPool::new_num_cpus());

    let objects = objects_from_path(objects)?;
    let objects = Arc::new(Mutex::new(objects));

    let index = index_from_path(index)?;
    let index = Arc::new(Mutex::new(index));

    let setup = move || {
        futures::future::ok::<_, hyper::Error>(reproto_service::ReprotoService {
            max_file_size,
            pool: pool.clone(),
            objects: objects.clone(),
            index: index.clone(),
        })
    };

    let server = Server::bind(&listen_address).serve(setup).map_err(|e| {
        error!("server error: {}", e);
    });

    info!("listening on http://{}", listen_address);
    rt::run(server);
    Ok(())
}

fn main() {
    if let Err(e) = entry() {
        error!("{}", e.message());

        for e in e.causes().skip(1) {
            error!("caused by: {}", e.message());
        }

        if let Some(backtrace) = e.backtrace() {
            error!("{:?}", backtrace);
        }
    }

    info!("Shutting down");
}
