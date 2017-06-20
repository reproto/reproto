mod collecting;
mod package_processor;
pub(crate) mod container;
pub(crate) mod converter;
pub(crate) mod decode;
pub(crate) mod doc;
pub(crate) mod dynamic_converter;
pub(crate) mod dynamic_decode;
pub(crate) mod dynamic_encode;
pub(crate) mod encode;
pub(crate) mod environment;
pub(crate) mod for_context;
pub(crate) mod java;
pub(crate) mod js;
pub(crate) mod json;
pub(crate) mod match_decode;
pub(crate) mod python;
pub(crate) mod rust;
pub(crate) mod value_builder;
pub(crate) mod variables;

use options::Options;
pub use self::environment::{Environment, InitFields};
pub(crate) use super::errors;
use super::errors::*;

pub trait Backend {
    fn process(&self) -> Result<()>;

    fn verify(&self) -> Result<Vec<Error>>;
}

pub fn resolve(backend: &str, options: Options, env: Environment) -> Result<Box<Backend>> {
    let backend: Box<Backend> = match backend {
        "java" => Box::new(java::resolve(options, env)?),
        "python" => Box::new(python::resolve(options, env)?),
        "js" => Box::new(js::resolve(options, env)?),
        "rust" => Box::new(rust::resolve(options, env)?),
        "doc" => Box::new(doc::resolve(options, env)?),
        "json" => Box::new(json::resolve(options, env)?),
        _ => return Err(format!("Unknown backend type: {}", backend).into()),
    };

    Ok(backend)
}
