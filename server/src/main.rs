#[macro_use]
extern crate log;
extern crate tempfile;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate pretty_env_logger;
extern crate reproto_server;
extern crate reproto_repository;

use futures_cpupool::CpuPool;
use hyper::server::Http;
use reproto_repository::objects_from_file;
use reproto_server::errors::*;
use reproto_server::reproto_service;
use std::path::Path;
use std::sync::{Arc, Mutex};

fn entry() -> Result<()> {
    pretty_env_logger::init()?;

    // TODO: get these from configuration
    let addr = "127.0.0.1:1234".parse()?;
    let path = Path::new("./objects");
    let max_file_size = 10000000;

    let pool = Arc::new(CpuPool::new_num_cpus());
    let setup_pool = pool.clone();
    let objects = Arc::new(Mutex::new(objects_from_file(path)?));

    let setup = move || {
        Ok(reproto_service::ReprotoService {
            max_file_size: max_file_size,
            pool: setup_pool.clone(),
            objects: objects.clone(),
        })
    };

    let server = Http::new().bind(&addr, setup)?;

    info!("Listening on http://{}", server.local_addr()?);
    server.run()?;
    Ok(())
}

fn main() {
    if let Err(e) = entry() {
        error!("ERROR - {}", e);
    }

    info!("Shutting down");
}
