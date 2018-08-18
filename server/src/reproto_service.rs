use errors::Error;
use flate2::read::GzDecoder;
use futures::future::{err, ok};
use futures_cpupool::CpuPool;
use hyper::header::{self, HeaderMap, HeaderValue};
use hyper::rt::{Future, Stream};
use hyper::service::Service;
use hyper::{self, Body, Method, Request, Response, StatusCode};
use reproto_repository::{to_checksum, Checksum, Index, Objects};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use tokio_fs;
use tokio_io;

const CHECKSUM_MISMATCH: &'static str = "checksum mismatch";

type BoxFut = Box<Future<Item = Response<Body>, Error = Error> + Send>;

pub struct ReprotoService {
    pub max_file_size: u64,
    pub pool: Arc<CpuPool>,
    pub objects: Arc<Mutex<Box<Objects>>>,
    pub index: Arc<Mutex<Box<Index>>>,
}

type EncodingFn = fn(&Cursor<Vec<u8>>) -> Box<Read>;

impl ReprotoService {
    fn no_encoding(input: &Cursor<Vec<u8>>) -> Box<Read> {
        Box::new(input.clone())
    }

    fn gzip_encoding(input: &Cursor<Vec<u8>>) -> Box<Read> {
        Box::new(GzDecoder::new(input.clone()))
    }

    fn pick_encoding(headers: &HeaderMap<HeaderValue>) -> EncodingFn {
        if let Some(value) = headers.get(header::CONTENT_ENCODING) {
            if value == "gzip" {
                return Self::gzip_encoding;
            }
        }

        Self::no_encoding
    }

    fn build_index(&self, m: &mut String) -> Result<(), Error> {
        use std::fmt::Write;

        writeln!(m, "<html>")?;
        writeln!(m, "<head>")?;
        writeln!(m, "<title>Reproto Repository</title>")?;
        writeln!(m, "</head>")?;
        writeln!(m, "<body>")?;
        writeln!(m, "</body>")?;
        writeln!(m, "</html>")?;

        Ok(())
    }

    fn get_index(&self) -> BoxFut {
        let mut m = String::new();

        if let Err(e) = self.build_index(&mut m) {
            return Box::new(err(e.into()));
        }

        let response = Response::new(Body::from(m));
        Box::new(ok(response))
    }

    /// Get an object from an object repository.
    fn get_objects(&self, id: &str) -> BoxFut {
        let checksum = match Checksum::from_str(id) {
            Ok(checksum) => checksum,
            Err(_) => {
                return Box::new(err(Error::BadRequest(
                    format!("bad object id: {}", id).into(),
                )))
            }
        };

        let mut objects = match self.objects.lock() {
            Ok(objects) => objects,
            Err(_) => return Box::new(err(Error::InternalServerError("lock poisoned".into()))),
        };

        let object = match objects.get_object(&checksum) {
            Ok(object) => object,
            Err(e) => return Box::new(err(e.into())),
        };

        let path = match object {
            Some(object) => match object.path() {
                Some(path) => path.to_owned(),
                None => {
                    return Box::new(err(Error::InternalServerError(
                        "object does not have a path".into(),
                    )))
                }
            },
            None => return Box::new(err(Error::NotFound)),
        };

        let fut = tokio_fs::file::File::open(path).or_else(|_| Box::new(err(Error::NotFound)));

        let fut = fut.and_then(|file| {
            let buf: Vec<u8> = Vec::new();

            let fut = tokio_io::io::read_to_end(file, buf)
                .and_then(|item| Ok(Response::new(item.1.into())))
                .or_else(|_| Err(Error::InternalServerError("error sending file".into())));

            Box::new(fut)
        });

        Box::new(fut)
    }

    /// Put the uploaded object into the object repository.
    fn put_uploaded_object(
        &self,
        body: impl 'static + Future<Item = Cursor<Vec<u8>>, Error = Error> + Send,
        checksum: Checksum,
        encoding: EncodingFn,
    ) -> BoxFut {
        let pool = self.pool.clone();
        let objects = self.objects.clone();

        let upload = body.and_then(move |mut tmp| {
            pool.spawn_fn(move || {
                if let Err(e) = tmp.seek(SeekFrom::Start(0)) {
                    return Box::new(err(e.into()));
                }

                let mut checksum_read = encoding(&tmp);

                let actual = match to_checksum(&mut checksum_read) {
                    Ok(actual) => actual,
                    Err(e) => return Box::new(err(e.into())),
                };

                if actual != checksum {
                    info!("{} != {}", actual, checksum);
                    return Box::new(err(Error::BadRequest(CHECKSUM_MISMATCH.into())));
                }

                info!("Uploading object: {}", checksum);

                if let Err(e) = tmp.seek(SeekFrom::Start(0)) {
                    return Box::new(err(e.into()));
                }

                let mut read = encoding(&tmp);

                let mut objects = match objects.lock() {
                    Ok(objects) => objects,
                    Err(_) => {
                        return Box::new(err(Error::InternalServerError("lock poisoned".into())))
                    }
                };

                if let Err(e) = objects.put_object(&checksum, &mut read, false) {
                    return Box::new(err(e.into()));
                }

                Box::new(ok(Response::new(Body::empty())))
            })
        });

        Box::new(upload)
    }

    fn put_objects(&self, id: &str, req: Request<Body>) -> BoxFut {
        let checksum = match Checksum::from_str(id) {
            Ok(checksum) => checksum,
            Err(e) => return Box::new(err(e.into())),
        };

        {
            let len = match req.headers().get(header::CONTENT_LENGTH) {
                Some(len) => len,
                None => return Box::new(err(Error::BadRequest("missing content-length".into()))),
            };

            let len = match len
                .to_str()
                .map_err(|_| "not a string")
                .and_then(|s| s.parse::<u64>().map_err(|_| "not a number"))
            {
                Ok(len) => len,
                Err(e) => {
                    return Box::new(err(Error::BadRequest(
                        format!("bad content-length: {}", e).into(),
                    )))
                }
            };

            if len > self.max_file_size {
                return Box::new(err(Error::BadRequest("file too large".into())));
            }
        }

        let encoding = Self::pick_encoding(req.headers());

        info!("Creating temporary file");

        let out = Vec::<u8>::new();

        let body = req.into_body().fold(out, |mut out, chunk| {
            out.extend(chunk.as_ref());
            ok::<_, hyper::Error>(out)
        });

        let body = body
            .map(Cursor::new)
            .map_err(|e| Error::InternalServerError(format!("error receiving body: {}", e).into()));

        self.put_uploaded_object(body, checksum, encoding)
    }

    fn inner_call<'a>(
        &self,
        req: Request<Body>,
        path: impl IntoIterator<Item = &'a str>,
    ) -> BoxFut {
        let mut it = path.into_iter();

        let a = it.next();
        let b = it.next();
        let c = it.next();

        match (req.method(), a, b, c) {
            (&Method::GET, Some(""), None, None) => {
                return self.get_index();
            }
            (&Method::GET, Some("objects"), Some(id), None) => {
                return self.get_objects(id);
            }
            (&Method::PUT, Some("objects"), Some(id), None) => {
                return self.put_objects(id, req);
            }
            _ => {}
        }

        Box::new(err(Error::NotFound))
    }

    fn handle_error(e: Error) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let mut response = Response::new(Body::empty());

        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        match e {
            Error::NotFound => {
                *response.body_mut() = Body::from("not found");
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
            Error::BadRequest(message) => {
                *response.body_mut() = Body::from(message);
                *response.status_mut() = StatusCode::BAD_REQUEST;
            }
            Error::InternalServerError(message) => {
                *response.body_mut() = Body::from("internal server error");
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

                error!("internal server error: {}", message);
            }
            Error::Core(error) => {
                *response.body_mut() = Body::from("internal server error");
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

                error!("internal server error: {}", error.message());

                for e in error.causes().skip(1) {
                    error!("caused by: {}", e.message());
                }

                if let Some(backtrace) = error.backtrace() {
                    error!("{:?}", backtrace);
                }
            }
        }

        return Box::new(ok(response));
    }
}

impl Service for ReprotoService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let full_path = String::from(req.uri().path());

        let path = full_path.split('/').skip(1);

        let fut = self
            .inner_call(req, path)
            .or_else(|e| Self::handle_error(e));

        Box::new(fut)
    }
}
