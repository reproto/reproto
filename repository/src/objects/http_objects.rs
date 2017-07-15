use errors::*;
use futures::{Future, Stream};
use futures::future::{err, ok};
use hex_slice::HexSlice;
use hyper;
use hyper::{Client, Method, Request, StatusCode};
use hyper::header::ContentLength;
use object::{BytesObject, Object, PathObject};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::time::{self, Duration};
use super::*;
use tokio_core::reactor::Core;
use url::Url;

pub struct HttpObjects {
    objects_cache: PathBuf,
    missing_cache_time: Duration,
    url: Url,
    core: Core,
}

impl HttpObjects {
    pub fn new(objects_cache: PathBuf,
               missing_cache_time: Duration,
               url: Url,
               core: Core)
               -> HttpObjects {
        HttpObjects {
            objects_cache: objects_cache,
            missing_cache_time: missing_cache_time,
            url: url,
            core: core,
        }
    }

    fn cache_path(&self, checksum: &Checksum) -> Result<PathBuf> {
        let path = self.objects_cache.join(format!("{}", HexSlice::new(&checksum[0..1])));
        let path = path.join(format!("{}", HexSlice::new(&checksum[1..2])));
        Ok(path.join(format!("{}", HexSlice::new(&checksum))))
    }

    /// Get the path to the missing file cache.
    fn missing_path(&self, checksum: &Checksum) -> Result<PathBuf> {
        Ok(self.objects_cache.join("missing").join(format!("{}", HexSlice::new(checksum))))
    }

    fn checksum_url(&self, checksum: &Checksum) -> Result<hyper::Uri> {
        let url = self.url.join(HexSlice::new(checksum).to_string().as_ref())?;
        let url = url.to_string().parse::<hyper::Uri>()?;
        Ok(url)
    }

    /// Check if there is a local missing cached file, and assume that the remote file is missing
    /// if it is present, or younger than `missing_cache_time`.
    ///
    /// Returns Some(cache_path) if the file might exist.
    fn check_missing(&self, checksum: &Checksum) -> Result<(bool, PathBuf)> {
        let path = self.missing_path(checksum)?;

        match fs::metadata(&path) {
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    return Err(e.into());
                }
            }
            Ok(m) => {
                let now = time::SystemTime::now();
                let age = now.duration_since(m.modified()?)?;

                let expires =
                    self.missing_cache_time.checked_sub(age).unwrap_or_else(|| Duration::new(0, 0));

                debug!("cache: missing file exists: {} (age: {}s, expires: {}s)",
                       path.display(),
                       age.as_secs(),
                       expires.as_secs());

                // remote file is expected to be missing
                if age < self.missing_cache_time {
                    return Ok((true, path));
                }

                debug!("cache: removing missing entry: {}", path.display());
                fs::remove_file(&path)?;
            }
        }

        Ok((false, path))
    }

    fn handle_request(&mut self,
                      request: Request)
                      -> Box<Future<Item = (Vec<u8>, StatusCode), Error = Error>> {
        let handle = self.core.handle();
        let client = Client::new(&handle);

        let body_and_status = client.request(request)
            .map_err::<_, Error>(Into::into)
            .and_then(|res| {
                let status = res.status().clone();

                res.body()
                    .map_err::<Error, _>(Into::into)
                    .fold(Vec::new(), |mut out: Vec<u8>, chunk| {
                        out.extend(chunk.as_ref());
                        ok::<_, Error>(out)
                    })
                    .map(move |body| (body, status))
            });

        Box::new(body_and_status)
    }
}

impl Objects for HttpObjects {
    fn put_object(&mut self, checksum: &Checksum, source: &mut Read, _force: bool) -> Result<()> {
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;

        let url = self.checksum_url(checksum)?;

        let mut request = Request::new(Method::Put, url);
        request.headers_mut().set(ContentLength(buffer.len() as u64));
        request.set_body(buffer);

        let work = self.handle_request(request).and_then(|(body, status)| {
            if !status.is_success() {
                if let Ok(body) = String::from_utf8(body) {
                    return err(format!("bad response: {}: {}", status, body).into());
                }

                return err(format!("bad response: {}", status).into());
            }

            ok(())
        });

        self.core.run(work)?;
        Ok(())
    }

    fn get_object(&mut self, checksum: &Checksum) -> Result<Option<Box<Object>>> {
        let cache_path = self.cache_path(checksum)?;

        if cache_path.is_file() {
            return Ok(Some(Box::new(PathObject::new(cache_path))));
        }

        let (missing, missing_path) = self.check_missing(checksum)?;

        if missing {
            return Ok(None);
        }

        let url = self.checksum_url(checksum)?;
        let name = url.to_string();

        let request = Request::new(Method::Get, url);

        let work = self.handle_request(request).and_then(|(body, status)| {
            if status.is_success() {
                return ok(Some(body));
            }

            if status == StatusCode::NotFound {
                return ok(None);
            }

            if let Ok(body) = String::from_utf8(body) {
                return err(format!("bad response: {}: {}", status, body).into());
            }

            return err(format!("bad response: {}", status).into());
        });

        let out = self.core.run(work)?;

        if let Some(bytes) = out {
            if let Some(parent) = cache_path.parent() {
                if !parent.is_dir() {
                    fs::create_dir_all(parent)?;
                }
            }

            File::create(cache_path)?.write_all(&bytes)?;
            return Ok(Some(Box::new(BytesObject::new(name, bytes)) as Box<Object>));
        } else {
            // write cache entry indicating that there is nothing in the remote entry to avoid
            // subsequent requests.
            debug!("cache: creating missing cache entry: {}",
                   missing_path.display());

            if let Some(parent) = missing_path.parent() {
                if !parent.is_dir() {
                    fs::create_dir_all(parent)?;
                }
            }

            File::create(missing_path)?;
        }

        return Ok(None);
    }
}
