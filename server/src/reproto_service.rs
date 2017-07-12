use errors::*;
use flate2::FlateReadExt;
use futures::Stream;
use futures::future::{BoxFuture, Future, ok};
use futures_cpupool::CpuPool;
use hyper::{self, Method, StatusCode};
use hyper::header::{ContentEncoding, ContentLength, Encoding, Headers};
use hyper::server::{Request, Response, Service};
use reproto_repository::{Checksum, Objects, to_checksum};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tempfile;

static CHECKSUM_MISMATCH: &'static [u8] = b"Checksum Mismatch";

/// ## Read the contents of the file into a byte-vector
fn read_contents<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)?;
    Ok(content)
}

pub struct ReprotoService {
    pub max_file_size: u64,
    pub pool: Arc<CpuPool>,
    pub objects: Arc<Mutex<Box<Objects>>>,
}

impl ReprotoService {
    fn no_encoding(input: &File) -> Result<Box<Read>> {
        Ok(Box::new(input.try_clone()?))
    }

    fn gzip_encoding(input: &File) -> Result<Box<Read>> {
        Ok(Box::new(input.try_clone()?.gz_decode()?))
    }

    fn pick_encoding(headers: &Headers) -> fn(&File) -> Result<Box<Read>> {
        if let Some(h) = headers.get::<ContentEncoding>() {
            // client encoded as gzip
            if h.0.iter().find(|encoding| **encoding == Encoding::Gzip).is_some() {
                return Self::gzip_encoding;
            }
        }

        Self::no_encoding
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

        let encoding = Self::pick_encoding(req.headers());

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

impl Service for ReprotoService {
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
