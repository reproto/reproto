#[macro_use]
extern crate log;
#[allow(unused)]
#[macro_use]
extern crate reproto_backend as backend;
extern crate reproto_core as core;
#[macro_use]
extern crate reproto_manifest as manifest;
extern crate reproto_trans as trans;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml as yaml;
extern crate toml;

const OPENAPI_VERSION: &str = "3.0.0";

use core::errors::*;
use core::{CoreFlavor, Handle, RelativePathBuf, RpDecl, RpHttpMethod, Version};
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use trans::Environment;

#[derive(Clone, Copy, Default, Debug)]
pub struct OpenApiLang;

impl Lang for OpenApiLang {
    lang_base!(OpenApiModule, compile);
}

#[derive(Debug)]
pub enum OpenApiModule {
}

impl TryFromToml for OpenApiModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

#[derive(Default, Debug, Serialize)]
struct Info<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a Version>,
}

#[derive(Default, Debug, Serialize)]
struct Method<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_id: Option<&'a str>,
}

#[derive(Default, Debug, Serialize)]
struct SpecPath<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post: Option<Method<'a>>,
}

#[derive(Debug, Serialize)]
struct Spec<'a> {
    openapi: &'static str,
    info: Info<'a>,
    servers: Vec<&'a str>,
    paths: HashMap<String, SpecPath<'a>>,
}

fn compile(handle: &Handle, env: Environment<CoreFlavor>, _manifest: Manifest) -> Result<()> {
    let env = env.translate_default()?;

    let root = RelativePathBuf::from(".");

    for (package, file) in env.for_each_file() {
        let mut path = package
            .package
            .parts()
            .fold(root.clone(), |path, part| path.join(part));

        let path = if let Some(version) = package.version.as_ref() {
            path.join(format!("v{}", version))
        } else {
            path
        };

        for d in file.for_each_decl() {
            // Use services as entrypoints.
            if let RpDecl::Service(ref service) = *d {
                if !handle.is_dir(&path) {
                    debug!("+dir: {}", path.display());
                    handle.create_dir_all(&path)?;
                }

                let path = path.join(&service.ident).with_extension("yaml");

                let mut spec = Spec {
                    openapi: OPENAPI_VERSION,
                    info: Info::default(),
                    servers: Vec::new(),
                    paths: HashMap::new(),
                };

                if let Some(version) = package.version.as_ref() {
                    spec.info.version = Some(version);
                }

                if let Some(ref url) = service.http.url {
                    spec.servers.push(url);
                }

                // NB: we need to group each path.
                for e in &service.endpoints {
                    let path = match e.http.path {
                        Some(ref path) => path,
                        None => continue,
                    };

                    let method = match e.http.method {
                        Some(ref method) => *method,
                        // TODO: handle during into_model transformation.
                        None => RpHttpMethod::Get,
                    };

                    let mut p = spec.paths
                        .entry(path.to_string())
                        .or_insert_with(SpecPath::default);

                    let method = match method {
                        RpHttpMethod::Get => &mut p.get,
                        RpHttpMethod::Post => &mut p.post,
                        m => return Err(format!("method `{:?}` is not supported", m).into()),
                    };

                    let method = method.get_or_insert_with(Method::default);
                    method.operation_id = Some(e.safe_ident());
                }

                debug!("+file: {}", path.display());
                writeln!(handle.create(&path)?, "{}", yaml::to_string(&spec)?)?;
            }
        }
    }

    Ok(())
}
