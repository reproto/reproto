mod into_model;
mod merge;
mod options;

pub mod environment;
pub mod errors;
pub mod for_context;
pub mod java;
pub mod js;
pub mod models;
pub mod python;

pub use self::environment::Environment;
use options::Options;
use self::errors::*;

pub trait Backend {
    fn process(&self) -> Result<()>;

    fn verify(&self) -> Result<Vec<Error>>;
}

pub fn resolve(backend: &str, options: Options, env: Environment) -> Result<Box<Backend>> {
    let backend: Box<Backend> = match backend {
        "java" => Box::new(java::resolve(options, env)?),
        "python" => Box::new(python::resolve(options, env)?),
        "js" => Box::new(js::resolve(options, env)?),
        _ => return Err(format!("Unknown backend type: {}", backend).into()),
    };

    Ok(backend)
}
