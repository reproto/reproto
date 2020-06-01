use crate::errors::Error;
use flate2::read::GzDecoder;
use futures::{Future, StreamExt};
use hyper::header::{self, HeaderMap, HeaderValue};
use hyper::service::Service;
use hyper::{self, Body, Method, Request, Response, StatusCode};
use reproto_repository::{to_checksum, Checksum, Index, Objects};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::io::AsyncReadExt;

const CHECKSUM_MISMATCH: &'static str = "checksum mismatch";

struct Inner {
    pub max_file_size: u64,
    pub objects: Mutex<Box<dyn Objects>>,
    pub index: Mutex<Box<dyn Index>>,
}

#[derive(Clone)]
pub struct ReprotoService {
    inner: Arc<Inner>,
}

type EncodingFn = fn(&Cursor<Vec<u8>>) -> Box<dyn Read>;

impl ReprotoService {
    pub fn new(
        max_file_size: u64,
        objects: Mutex<Box<dyn Objects>>,
        index: Mutex<Box<dyn Index>>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                max_file_size,
                objects,
                index,
            }),
        }
    }

    fn no_encoding(input: &Cursor<Vec<u8>>) -> Box<dyn Read> {
        Box::new(input.clone())
    }

    fn gzip_encoding(input: &Cursor<Vec<u8>>) -> Box<dyn Read> {
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

    async fn get_index(&self) -> Result<Response<Body>, Error> {
        let mut m = String::new();
        self.build_index(&mut m)?;
        let response = Response::new(Body::from(m));
        Ok(response)
    }

    /// Get an object from an object repository.
    async fn get_objects(&self, id: &str) -> Result<Response<Body>, Error> {
        let checksum = match Checksum::from_str(id) {
            Ok(checksum) => checksum,
            Err(_) => {
                return Err(Error::BadRequest(format!("bad object id: {}", id).into()));
            }
        };

        let path = {
            let mut objects = match self.inner.objects.lock() {
                Ok(objects) => objects,
                Err(_) => return Err(Error::InternalServerError("lock poisoned".into())),
            };

            let object = objects.get_object(&checksum)?;

            match object {
                Some(object) => match object.path() {
                    Some(path) => path.to_owned(),
                    None => {
                        return Err(Error::InternalServerError(
                            "object does not have a path".into(),
                        ));
                    }
                },
                None => return Err(Error::NotFound),
            }
        };

        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|_| Error::NotFound)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)
            .await
            .map_err(|_| Error::InternalServerError("error sending file".into()))?;
        Ok(Response::new(Body::from(buf)))
    }

    /// Put the uploaded object into the object repository.
    async fn put_uploaded_object(
        &self,
        mut body: Cursor<Vec<u8>>,
        checksum: Checksum,
        encoding: EncodingFn,
    ) -> Result<Response<Body>, Error> {
        let inner = self.inner.clone();

        let result = tokio::task::spawn_blocking(move || {
            body.seek(SeekFrom::Start(0))?;

            let mut checksum_read = encoding(&body);

            let actual = to_checksum(&mut checksum_read)?;

            if actual != checksum {
                info!("{} != {}", actual, checksum);
                return Err(Error::BadRequest(CHECKSUM_MISMATCH.into()));
            }

            info!("Uploading object: {}", checksum);

            body.seek(SeekFrom::Start(0))?;

            let mut read = encoding(&body);

            let mut objects = match inner.objects.lock() {
                Ok(objects) => objects,
                Err(_) => {
                    return Err(Error::InternalServerError("lock poisoned".into()));
                }
            };

            objects.put_object(&checksum, &mut read, false)?;
            Ok::<Response<Body>, Error>(Response::new(Body::empty()))
        })
        .await;

        result.map_err(|_| Error::InternalServerError("task failed".into()))?
    }

    async fn put_objects(&self, id: &str, mut req: Request<Body>) -> Result<Response<Body>, Error> {
        let checksum = Checksum::from_str(id)?;

        {
            let len = match req.headers().get(header::CONTENT_LENGTH) {
                Some(len) => len,
                None => return Err(Error::BadRequest("missing content-length".into())),
            };

            let len = match len
                .to_str()
                .map_err(|_| "not a string")
                .and_then(|s| s.parse::<u64>().map_err(|_| "not a number"))
            {
                Ok(len) => len,
                Err(e) => {
                    return Err(Error::BadRequest(
                        format!("bad content-length: {}", e).into(),
                    ));
                }
            };

            if len > self.inner.max_file_size {
                return Err(Error::BadRequest("file too large".into()));
            }
        }

        let encoding = Self::pick_encoding(req.headers());

        info!("Creating temporary file");

        let mut out = Vec::<u8>::new();
        let body = req.body_mut();

        while let Some(chunk) = body.next().await.transpose()? {
            out.extend(&chunk);
        }

        self.put_uploaded_object(Cursor::new(out), checksum, encoding)
            .await
    }

    async fn inner_call<'a>(
        &self,
        req: Request<Body>,
        path: impl IntoIterator<Item = &str>,
    ) -> Result<Response<Body>, Error> {
        let mut it = path.into_iter();

        let a = it.next();
        let b = it.next();
        let c = it.next();

        match (req.method(), a, b, c) {
            (&Method::GET, Some(""), None, None) => {
                return self.get_index().await;
            }
            (&Method::GET, Some("objects"), Some(id), None) => {
                return self.get_objects(id).await;
            }
            (&Method::PUT, Some("objects"), Some(id), None) => {
                return self.put_objects(id, req).await;
            }
            _ => {}
        }

        Err(Error::NotFound)
    }

    fn handle_error(e: Error) -> Response<Body> {
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

        response
    }
}

impl Service<Request<Body>> for ReprotoService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + 'static + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let this = self.clone();

        Box::pin(async move {
            let path = req.uri().path().to_owned();

            match this.inner_call(req, path.split('/').skip(1)).await {
                Ok(response) => Ok(response),
                Err(e) => Ok(Self::handle_error(e)),
            }
        })
    }
}
