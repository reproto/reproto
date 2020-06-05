//! ## Load objects from a remote repository over HTTP

use core::errors::Result;
use core::Source;
use futures::future::{err, ok};
use futures::{StreamExt as _, TryFutureExt as _};
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

    async fn handle_request(&mut self, request: Request<Body>) -> Result<(Vec<u8>, StatusCode)> {
        let mut res = self.client.request(request).await?;

        let body = res.body_mut();
        let mut output = Vec::new();

        while let Some(chunk) = body.next().await {
            let bytes = chunk?;
            output.extend(&bytes[..]);
        }

        Ok((output, res.status()))
    }
}

impl Objects for HttpObjects {
    fn put_object(
        &mut self,
        checksum: &Checksum,
        source: &mut dyn Read,
        _force: bool,
    ) -> Result<bool> {
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

        futures::executor::block_on(work)?;

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

        let out = futures::executor::block_on(work)?;
        Ok(out.map(|out| Source::bytes(name, out)))
    }
}

/// Load objects from an HTTP url.
pub fn objects_from_url(config: ObjectsConfig, url: &Url) -> Result<Box<dyn Objects>> {
    let client = Client::builder().build(HttpsConnector::new());

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
