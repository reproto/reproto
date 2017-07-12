#[macro_use]
extern crate log;
extern crate tempfile;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate pretty_env_logger;
extern crate reproto_repository;
extern crate reproto_server;
extern crate flate2;

use flate2::FlateReadExt;
use futures::Stream;
use futures::future::{BoxFuture, Future, ok};
use futures_cpupool::CpuPool;
use hyper::{Method, StatusCode};
use hyper::header::{ContentEncoding, ContentLength, Encoding};
use hyper::server::{Http, Request, Response, Service};
use reproto_repository::{Checksum, Objects, objects_from_file, to_checksum};
use reproto_server::errors::*;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::sync::{Arc, Mutex};

static CHECKSUM_MISMATCH: &'static [u8] = b"Checksum Mismatch";

/// ## Read the contents of the file into a byte-vector
fn read_contents<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)?;
    Ok(content)
}

struct ReprotoServer {
    max_file_size: u64,
    pool: Arc<CpuPool>,
    objects: Arc<Mutex<Box<Objects>>>,
}

impl ReprotoServer {
    fn no_encoding(input: &File) -> Result<Box<Read>> {
        Ok(Box::new(input.try_clone()?))
    }

    fn gzip_encoding(input: &File) -> Result<Box<Read>> {
        Ok(Box::new(input.try_clone()?.gz_decode()?))
    }

    fn not_found() -> Response {
        Response::new().with_status(StatusCode::NotFound)
    }

    fn get_objects(&self, path: &str) -> Result<BoxFuture<Response, Error>> {
        let objects = self.objects.clone();

        let checksum =
            Checksum::from_str(&path[..]).map_err(|_| ErrorKind::BadRequest("bad object id"))?;

        // No async I/O, use pool
        Ok(self.pool
            .spawn_fn(move || {
                let result =
                    objects.lock().map_err(|_| ErrorKind::PoisonError)?.get_object(&checksum)?;

                let path = match result {
                    Some(path) => path,
                    None => return Ok(Self::not_found()),
                };

                let bytes = read_contents(path)?;

                Ok(Response::new()
                    .with_status(StatusCode::Ok)
                    .with_header(ContentLength(bytes.len() as u64))
                    .with_body(bytes))
            })
            .boxed())
    }

    fn put_objects(&self, req: Request, path: &str) -> Result<BoxFuture<Response, Error>> {
        let checksum =
            Checksum::from_str(&path[..]).map_err(|_| ErrorKind::BadRequest("bad object id"))?;

        if let Some(len) = req.headers().get::<ContentLength>() {
            if len.0 > self.max_file_size {
                return Err(ErrorKind::BadRequest("file too large").into());
            }
        } else {
            return Err(ErrorKind::BadRequest("missing content-length").into());
        }

        let mut encoding: fn(&File) -> Result<Box<Read>> = Self::no_encoding;

        if let Some(h) = req.headers().get::<ContentEncoding>() {
            // client encoded as gzip
            if h.0.iter().find(|encoding| **encoding == Encoding::Gzip).is_some() {
                encoding = Self::gzip_encoding;
            }
        }

        info!("Creating temporary file");
        /// TODO: make temporary
        let tmp = tempfile::tempfile()?;

        let pool = self.pool.clone();

        /// Write file in chunks as it becomes available
        let body = req.body()
            .map_err::<Error, _>(Into::into)
            .fold((pool, tmp), |(pool, mut tmp), chunk| {
                /// Write chunks on cpu-pool
                let write = pool.spawn_fn(move || {
                    tmp.write_all(chunk.as_ref())?;
                    Ok(tmp) as Result<File>
                });

                write.map(|tmp| (pool, tmp))
            })
            .map(|(_, tmp)| tmp);

        let pool = self.pool.clone();
        let objects = self.objects.clone();

        let upload = body.and_then(move |mut tmp| {
            pool.spawn_fn(move || {
                tmp.seek(SeekFrom::Start(0))?;

                let mut checksum_read = encoding(&tmp)?;
                let actual = to_checksum(&mut checksum_read)?;

                if actual != checksum {
                    info!("{} != {}", actual, checksum);

                    return Ok(Response::new()
                        .with_body(CHECKSUM_MISMATCH)
                        .with_status(StatusCode::BadRequest));
                }

                info!("Uploading object: {}", checksum);

                let mut read = encoding(&tmp)?;

                objects.lock()
                    .map_err(|_| ErrorKind::PoisonError)?
                    .put_object(&checksum, &mut read)?;

                Ok(Response::new().with_status(StatusCode::Ok))
            })
        });

        Ok(upload.boxed())
    }

    fn inner_call(&self, req: Request, path: &str) -> Result<BoxFuture<Response, Error>> {
        if let Some(len) = path.find('/') {
            match (req.method(), &path[0..len]) {
                (&Method::Get, "objects") => return self.get_objects(&path[len + 1..]),
                (&Method::Put, "objects") => return self.put_objects(req, &path[len + 1..]),
                _ => return Ok(ok(Self::not_found()).boxed()),
            }
        }

        Ok(ok(Self::not_found()).boxed())
    }

    fn handle_error(e: Error) -> Response {
        match *e.kind() {
            ErrorKind::BadRequest(ref message) => {
                return Response::new()
                    .with_status(StatusCode::BadRequest)
                    .with_header(ContentLength(message.len() as u64))
                    .with_body(*message)
            }
            _ => {
                return Response::new().with_status(StatusCode::InternalServerError);
            }
        }
    }
}

impl Service for ReprotoServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut path = String::from(req.path());

        // remove leading slash
        if let Some(len) = path.find('/') {
            path.drain(..len + 1);
        }

        self.inner_call(req, path.as_str())
            .unwrap_or_else(|e| ok(Self::handle_error(e)).boxed())
            .or_else(|e| ok(Self::handle_error(e)))
            .boxed()
    }
}

fn entry() -> Result<()> {
    pretty_env_logger::init()?;

    let addr = "127.0.0.1:1234".parse()?;

    let pool = Arc::new(CpuPool::new_num_cpus());
    let setup_pool = pool.clone();

    let path = Path::new("./objects");

    let objects = Arc::new(Mutex::new(objects_from_file(path)?));

    let max_file_size = 10000000;

    let setup = move || {
        Ok(ReprotoServer {
            max_file_size: max_file_size,
            pool: setup_pool.clone(),
            objects: objects.clone(),
        })
    };

    let server = Http::new().bind(&addr, setup)?;
    println!("Listening on http://{} with 1 thread.",
             server.local_addr()?);
    server.run()?;
    Ok(())
}

fn main() {
    if let Err(e) = entry() {
        println!("ERROR - {}", e);
    }

    println!("Shutting down");
}
