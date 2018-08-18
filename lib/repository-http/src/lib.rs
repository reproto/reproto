//! ## Load objects from a remote repository over HTTP

extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate reproto_core as core;
extern crate reproto_repository as repository;
extern crate url;

use core::errors::{Error, Result};
use core::Source;
use futures::future::{err, ok};
use futures::{Future, Stream};
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_rustls::HttpsConnector;
use repository::{CachedObjects, Checksum, HexSlice, Objects, ObjectsConfig};
use std::io::Read;
use std::time::Duration;
use url::Url;

pub struct HttpObjects {
    url: Url,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl HttpObjects {
    fn checksum_url(&self, checksum: &Checksum) -> Result<hyper::Uri> {
        let url = self
            .url
            .join(HexSlice::new(checksum).to_string().as_ref())?;

        let url = url
            .to_string()
            .parse::<hyper::Uri>()
            .map_err(|e| format!("Failed to parse URL: {}: {}", e, url))?;

        Ok(url)
    }

    fn handle_request(
        &mut self,
        request: Request<Body>,
    ) -> impl Future<Item = (Vec<u8>, StatusCode), Error = Error> {
        let body_and_status = self
            .client
            .request(request)
            .map_err::<_, Error>(|e| format!("Request to repository failed: {}", e).into())
            .and_then(|res| {
                let status = res.status().clone();

                res.into_body()
                    .map_err::<Error, _>(|e| format!("Failed to perform request: {}", e).into())
                    .fold(Vec::new(), |mut out: Vec<u8>, chunk| {
                        out.extend(chunk.as_ref());
                        ok::<_, Error>(out)
                    }).map(move |body| (body, status))
            });

        Box::new(body_and_status)
    }
}

impl Objects for HttpObjects {
    fn put_object(&mut self, checksum: &Checksum, source: &mut Read, _force: bool) -> Result<bool> {
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;

        let url = self.checksum_url(checksum)?;

        let request = Request::builder()
            .method(Method::PUT)
            .uri(url)
            .body(Body::from(buffer))?;

        let work = self.handle_request(request).and_then(|(body, status)| {
            if !status.is_success() {
                if let Ok(body) = String::from_utf8(body) {
                    return err(format!("bad response: {}: {}", status, body).into());
                }

                return err(format!("bad response: {}", status).into());
            }

            ok(())
        });

        work.wait()?;

        // TODO: use status code to determine if the upload resulted in changes or not.
        Ok(true)
    }

    fn get_object(&mut self, checksum: &Checksum) -> Result<Option<Source>> {
        let url = self.checksum_url(checksum)?;
        let name = url.to_string();

        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::empty())?;

        let work = self.handle_request(request).and_then(|(body, status)| {
            if status.is_success() {
                return ok(Some(body));
            }

            if status == StatusCode::NOT_FOUND {
                return ok(None);
            }

            if let Ok(body) = String::from_utf8(body) {
                return err(format!("bad response: {}: {}", status, body).into());
            }

            return err(format!("bad response: {}", status).into());
        });

        let out = work.wait()?;
        Ok(out.map(|out| Source::bytes(name, out)))
    }
}

/// Load objects from an HTTP url.
pub fn objects_from_url(config: ObjectsConfig, url: &Url) -> Result<Box<Objects>> {
    let client = Client::builder().build(HttpsConnector::new(4));

    let http_objects = HttpObjects {
        url: url.clone(),
        client,
    };

    if let Some(cache_home) = config.cache_home {
        let missing_cache_time = config
            .missing_cache_time
            .unwrap_or_else(|| Duration::new(60, 0));

        return Ok(Box::new(CachedObjects::new(
            cache_home,
            missing_cache_time,
            http_objects,
        )));
    }

    Ok(Box::new(http_objects))
}
