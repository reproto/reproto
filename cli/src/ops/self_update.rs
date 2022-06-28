//! build command
use clap::{App, Arg, SubCommand};

#[cfg(feature = "self-updates")]
mod internal {
    use crate::VERSION;
    use clap::ArgMatches;
    use flate2::read::GzDecoder;
    use futures::{executor, Future, StreamExt};
    use hyper::client::HttpConnector;
    use hyper::header;
    use hyper::{Body, Client, Method, Request, Response, StatusCode, Uri};
    use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
    use reproto_core::errors::{Error, Result};
    use reproto_core::Version;
    use std::fs::{self, File};
    use std::io::{self, Cursor};
    use std::path::Path;
    use std::pin::Pin;
    use std::sync::Arc;
    use tar::Archive;
    use url::Url;

    #[cfg(target_os = "macos")]
    mod os {
        pub const PLATFORM: Option<&str> = Some("osx");
        pub const EXT: Option<&str> = Some("");
    }

    #[cfg(target_os = "linux")]
    mod os {
        pub const PLATFORM: Option<&str> = Some("linux");
        pub const EXT: Option<&str> = Some("");
    }

    #[cfg(target_os = "windows")]
    mod os {
        pub const PLATFORM: Option<&str> = Some("windows");
        pub const EXT: Option<&str> = Some(".exe");
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    mod os {
        pub const PLATFORM: Option<&str> = None;
        pub const EXT: Option<&str> = None;
    }

    #[cfg(target_arch = "x86_64")]
    const ARCH: Option<&str> = Some("x86_64");

    #[cfg(not(target_arch = "x86_64"))]
    const ARCH: Option<&str> = None;

    use self::os::EXT;
    use self::os::PLATFORM;

    const DEFAULT_URL: &str = "https://storage.googleapis.com/reproto-releases/";

    pub fn entry(m: &ArgMatches) -> Result<()> {
        let config = env::ConfigEnvironment::new()?
            .ok_or_else(|| format!("could not setup the reproto session"))?;

        check_path(&config)?;

        let force = m.try_contains_id("force").unwrap_or_default();
        let prerelease = m.try_contains_id("prerelease").unwrap_or_default();

        let arch = ARCH
            .clone()
            .or(m
                .try_get_one::<String>("arch")
                .ok()
                .and_then(|s| Some(s?.as_str())))
            .ok_or_else(|| {
                "Architecture could not be detected, and is not specified with `--arch`"
            })?;

        let platform = PLATFORM
            .clone()
            .or(m
                .try_get_one::<String>("platform")
                .ok()
                .and_then(|s| Some(s?.as_str())))
            .ok_or_else(|| {
                "Platform could not be detected, and is not specified with `--platform`"
            })?;

        let ext = EXT
            .clone()
            .or(m
                .try_get_one::<String>("ext")
                .ok()
                .and_then(|s| Some(s?.as_str())))
            .ok_or_else(|| {
                "Binary could not be detected, and is not specified with `--ext`. Should be \
                 something like `reproto` or `reproto.exe`"
            })?;

        let current = Version::parse(VERSION)?;

        let url = m
            .try_get_one::<String>("url")
            .ok()
            .and_then(|url| Some(url?.as_str()))
            .unwrap_or(DEFAULT_URL);
        let url = Url::parse(url)?;

        let mut client = UpdateClient::new(url)?;

        let mut releases = executor::block_on(client.get_releases())?;

        releases.sort();

        let releases = match prerelease {
            true => releases,
            false => releases
                .into_iter()
                .filter(|r| !r.is_prerelease())
                .collect(),
        };

        let candidate = if !force {
            releases.into_iter().filter(|r| r > &current).last()
        } else {
            releases.into_iter().last()
        };

        let version = match candidate {
            Some(version) => version,
            None => {
                log::info!("reproto is up-to-date!");
                return Ok(());
            }
        };

        let tuple = format!("{}-{}-{}", version, platform, arch);

        let archived = config
            .releases_dir
            .join(format!("reproto-{}{}", tuple, ext));
        let binary = format!("reproto{}", ext);

        if !archived.is_file() || force {
            let release = executor::block_on(client.get_release(&version, platform, arch))
                .map_err(|e| format!("{}: failed to download archive: {}", tuple, e.display()))?;

            download_archive(release, &archived, &binary).map_err(|e| {
                format!(
                    "failed to download archive to: {}: {}",
                    archived.display(),
                    e.display()
                )
            })?;

            log::info!("wrote: {}", archived.display());
        }

        let bin = config.bin_home.join(binary);

        setup_symlink(&archived, &bin).map_err(|e| {
            format!(
                "failed to setup symlink to: {}: {}",
                bin.display(),
                e.display()
            )
        })?;

        return Ok(());

        /// Checks that bin_home is in PATH, or warns otherwise.
        fn check_path(config: &env::ConfigEnvironment) -> Result<()> {
            let mut bin_in_path = false;

            if config.bin_home.is_dir() {
                if let Some(paths) = ::std::env::var_os("PATH") {
                    for path in ::std::env::split_paths(&paths) {
                        if !path.is_dir() {
                            continue;
                        }

                        if same_file::is_same_file(path, &config.bin_home)? {
                            bin_in_path = true;
                        }
                    }
                }
            }

            if !bin_in_path {
                log::warn!(
                    "{}: is not in your PATH. This is required for reproto to work!",
                    config.bin_home.display()
                );
            }

            Ok(())
        }

        fn download_archive(release: Vec<u8>, out: &Path, binary: &str) -> Result<()> {
            let mut archive = Archive::new(GzDecoder::new(Cursor::new(release)));

            for file in archive.entries()? {
                let mut file = file?;

                {
                    let path_bytes = file.header().path_bytes();
                    let path = ::std::str::from_utf8(path_bytes.as_ref())?;

                    if path != binary {
                        log::warn!("got unexpected file in archive: {}", path);
                        continue;
                    }
                }

                if let Some(parent) = out.parent() {
                    if !parent.is_dir() {
                        log::info!("creating directory: {}", parent.display());
                        fs::create_dir_all(parent)?;
                    }
                }

                log::info!("writing: {}", out.display());
                let mut w = File::create(out)?;
                io::copy(&mut file, &mut w)?;

                let metadata = w.metadata()?;
                let mut perm = metadata.permissions();
                set_executable(&mut perm);
                w.set_permissions(perm)?;
                break;
            }

            return Ok(());

            #[cfg(any(target_os = "linux", target_os = "macos"))]
            fn set_executable(p: &mut fs::Permissions) {
                use std::os::unix::fs::PermissionsExt;
                p.set_mode(0755);
            }

            #[cfg(target_os = "windows")]
            fn set_executable(_p: &mut fs::Permissions) {
                // nothing to do on windows.
            }

            #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
            fn set_executable(_p: &mut fs::Permissions) {
                log::warn!("cannot update permissions on this platform");
            }
        }

        fn setup_symlink(archived: &Path, bin: &Path) -> Result<()> {
            if let Some(parent) = bin.parent() {
                if !parent.is_dir() {
                    log::info!("creating directory: {}", parent.display());
                    fs::create_dir_all(parent)?;
                }
            }

            log::info!("creating symlink: {}", bin.display());

            if bin.is_file() {
                fs::remove_file(bin)?;
            }

            symlink(archived, bin)?;
            return Ok(());

            #[cfg(not(target_os = "windows"))]
            fn symlink(archived: &Path, bin: &Path) -> Result<()> {
                use std::os::unix;
                unix::fs::symlink(archived, bin)?;
                Ok(())
            }

            #[cfg(target_os = "windows")]
            fn symlink(archived: &Path, bin: &Path) -> Result<()> {
                use std::os::windows;
                windows::fs::symlink_file(archived, bin)?;
                Ok(())
            }
        }
    }

    struct UpdateClient {
        client: Arc<Client<HttpsConnector<HttpConnector>>>,
        url: Url,
    }

    impl UpdateClient {
        pub fn new(url: Url) -> Result<Self> {
            let client = Client::builder().build(
                HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_only()
                    .enable_http1()
                    .build(),
            );

            Ok(Self {
                client: Arc::new(client),
                url,
            })
        }

        /// Handle redirects for request.
        fn handle_redirect(
            client: Arc<Client<HttpsConnector<HttpConnector>>>,
            res: &Response<Body>,
        ) -> Result<Option<Pin<Box<dyn Future<Output = Result<Vec<u8>>>>>>> {
            let should_redirect = match res.status() {
                StatusCode::MOVED_PERMANENTLY
                | StatusCode::FOUND
                | StatusCode::SEE_OTHER
                | StatusCode::TEMPORARY_REDIRECT
                | StatusCode::PERMANENT_REDIRECT => true,
                _ => false,
            };

            let uri = if let Some(loc) = res.headers().get(header::LOCATION) {
                let s = ::std::str::from_utf8(loc.as_bytes()).map_err(Error::from)?;
                s.parse::<Uri>().map_err(Error::from)?
            } else {
                return Err("missing location header".into());
            };

            if !should_redirect {
                return Ok(None);
            }

            Ok(Some(Box::pin(async move {
                let req = Request::builder()
                    .method(Method::GET)
                    .uri(uri)
                    .body(Body::empty())?;

                Self::request(client, req).await
            })))
        }

        /// Perform the given request.
        async fn request(
            client: Arc<Client<HttpsConnector<HttpConnector>>>,
            req: Request<Body>,
        ) -> Result<Vec<u8>> {
            let inner_client = Arc::clone(&client);

            let mut res = client.request(req).await.map_err(Error::from)?;

            if let Some(future) = Self::handle_redirect(inner_client, &res)? {
                return future.await;
            }

            let status = res.status().clone();
            let body = res.body_mut();

            let mut output = Vec::new();

            while let Some(chunk) = body.next().await.transpose()? {
                output.extend(&chunk);
            }

            if !res.status().is_success() {
                if let Ok(body) = std::str::from_utf8(&output) {
                    return Err(format!("bad response: {}: {}", status, body).into());
                }

                return Err(format!("bad response: {}", status).into());
            }

            Ok(output)
        }

        pub async fn get_releases(&mut self) -> Result<Vec<Version>, Error> {
            let url = self.url.join("releases")?;
            let uri = url.as_ref().parse::<Uri>()?;
            let request = Request::get(uri).body(Body::empty())?;

            let url = url.clone();

            let body = Self::request(Arc::clone(&self.client), request)
                .await
                .map_err(move |e| format!("request to `{}` failed: {}", url, e.display()))?;

            let body = match String::from_utf8(body) {
                Ok(body) => body,
                Err(e) => return Err(format!("body is not utf-8: {}", e).into()),
            };

            let mut out = Vec::new();

            for line in body.split('\n') {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                out.push(Version::parse(line)?);
            }

            Ok(out)
        }

        pub async fn get_release(
            &mut self,
            version: &Version,
            platform: &str,
            arch: &str,
        ) -> Result<Vec<u8>, Error> {
            let url = self
                .url
                .join(&format!("reproto-{}-{}-{}.tar.gz", version, platform, arch))?;

            let uri = url.as_ref().parse::<Uri>()?;

            let request = Request::builder()
                .method(Method::GET)
                .uri(uri)
                .body(Body::empty())?;

            let url = url.clone();

            Self::request(Arc::clone(&self.client), request)
                .await
                .map_err(move |e| format!("request to `{}` failed: {}", url, e.display()).into())
        }
    }
}

#[cfg(not(feature = "self-updates"))]
mod internal {
    use clap::ArgMatches;
    use reproto_core::errors::Result;

    pub fn entry(_: &ArgMatches) -> Result<()> {
        return Err("support for self-updates is not enabled".into());
    }
}

pub use self::internal::entry;

pub fn options<'a>() -> App<'a> {
    let out = SubCommand::with_name("self-update").about("Update reproto");

    let out = out.arg(
        Arg::with_name("url")
            .long("url")
            .takes_value(true)
            .help("URL to download updates from."),
    );

    let out = out.arg(
        Arg::with_name("arch")
            .long("arch")
            .takes_value(true)
            .help("Architecture to update for"),
    );

    let out = out.arg(
        Arg::with_name("platform")
            .long("platform")
            .takes_value(true)
            .help("Architecture to update for"),
    );

    let out =
        out.arg(Arg::with_name("ext").long("ext").takes_value(true).help(
            "File extension of binary to expect, should be something like `.exe` on windows.",
        ));

    let out = out.arg(
        Arg::with_name("force")
            .short('f')
            .long("force")
            .help("Force downloading the latest release, even though it is already installed"),
    );

    let out = out.arg(
        Arg::with_name("prerelease")
            .long("pre")
            .help("Download a pre-release if available"),
    );

    out
}
