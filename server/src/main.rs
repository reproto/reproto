#[macro_use]
extern crate log;
extern crate reproto_core as core;
extern crate reproto_repository as repository;
extern crate reproto_server as server;

use core::errors::Result;
use hyper::server::Server;
use hyper::service::make_service_fn;
use repository::{index_from_path, objects_from_path};
use server::reproto_service;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

/// Get the configuration path to load.
fn config_path() -> Result<Option<PathBuf>> {
    use self::env::VarError;

    match env::var("REPROTO_SERVER_CONFIG") {
        Ok(path) => Ok(Some(Path::new(path.as_str()).to_owned())),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => return Err(e.into()),
    }
}

async fn entry() -> Result<()> {
    pretty_env_logger::init();

    let config = if let Some(path) = config_path()? {
        server::config::read_config(path)?
    } else {
        server::config::Config::default()
    };

    let server::config::Config {
        listen_address,
        objects,
        index,
        max_file_size,
    } = config;
    let listen_address = listen_address.parse()?;

    let objects = objects_from_path(objects)?;
    let objects = Mutex::new(objects);

    let index = index_from_path(index)?;
    let index = Mutex::new(index);

    let service = reproto_service::ReprotoService::new(max_file_size, objects, index);

    let setup = make_service_fn(move |_| {
        let service = service.clone();

        async { Ok::<_, hyper::Error>(service) }
    });

    info!("listening on http://{}", listen_address);
    let server = Server::bind(&listen_address).serve(setup);
    server.await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = entry().await {
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
