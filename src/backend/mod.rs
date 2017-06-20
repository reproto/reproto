mod base_decode;
mod base_encode;
mod collecting;
mod container;
mod converter;
mod doc;
mod dynamic_converter;
mod dynamic_decode;
mod dynamic_encode;
mod environment;
mod for_context;
mod java;
mod js;
mod json;
mod match_decode;
mod package_processor;
mod package_utils;
mod python;
mod rust;
mod value_builder;
mod variables;

pub(crate) use core::*;
pub(crate) use errors::*;
pub(crate) use options::Options;
pub(crate) use self::base_decode::*;
pub(crate) use self::base_encode::*;
pub(crate) use self::collecting::*;
pub(crate) use self::container::Container;
pub(crate) use self::converter::*;
pub(crate) use self::dynamic_converter::*;
pub(crate) use self::dynamic_decode::*;
pub(crate) use self::dynamic_encode::*;
pub use self::environment::{Environment, InitFields};
pub(crate) use self::match_decode::*;
pub(crate) use self::package_processor::*;
pub(crate) use self::package_utils::*;
pub(crate) use self::value_builder::*;
pub(crate) use self::variables::*;
use std::path::PathBuf;
pub(crate) use super::errors;

pub struct CompilerOptions {
    pub out_path: PathBuf,
}

pub trait Compiler<'a> {
    fn compile(&self) -> Result<()>;
}

pub trait Backend {
    fn compiler<'a>(&'a self, options: CompilerOptions) -> Result<Box<Compiler<'a> + 'a>>;

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
