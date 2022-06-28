//! ## Load objects from a remote repository over HTTP

use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use repository::{CachedObjects, Checksum, HexSlice, Objects, ObjectsConfig};
use reproto_core::errors::{Error, Result};
use reproto_core::Source;
use std::io::Read;
use std::time::Duration;
use tokio_stream::StreamExt;
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

        futures_executor::block_on(async {
            let (body, status) = self.handle_request(request).await?;

            if !status.is_success() {
                if let Ok(body) = String::from_utf8(body) {
                    return Err(format!("bad response: {}: {}", status, body).into());
                }

                return Err(format!("bad response: {}", status).into());
            }

            Ok::<_, Error>(())
        })?;

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

        let out = futures_executor::block_on(async {
            let (body, status) = self.handle_request(request).await?;

            if status.is_success() {
                return Ok::<_, Error>(Some(body));
            }

            if status == StatusCode::NOT_FOUND {
                return Ok(None);
            }

            if let Ok(body) = String::from_utf8(body) {
                return Err(format!("bad response: {}: {}", status, body).into());
            }

            return Err(format!("bad response: {}", status).into());
        })?;
        Ok(out.map(|out| Source::bytes(name, out)))
    }
}

/// Load objects from an HTTP url.
pub fn objects_from_url(config: ObjectsConfig, url: &Url) -> Result<Box<dyn Objects>> {
    let client = Client::builder().build(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build(),
    );

    let http_objects = HttpObjects {
        url: url.clone(),
        client,
    };

    let missing_cache_time = config
        .missing_cache_time
        .unwrap_or_else(|| Duration::new(60, 0));

    return Ok(Box::new(CachedObjects::new(
        config.cache_home,
        missing_cache_time,
        http_objects,
    )));
}
