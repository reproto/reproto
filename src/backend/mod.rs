pub mod java;
pub mod python;

use environment::Environment;
use options::Options;
use parser::ast;

use errors::*;

pub type TypeId = (ast::Package, String);

pub trait Backend {
    fn process(&self) -> Result<()>;
}

pub fn resolve(backend: &str, options: Options, env: Environment) -> Result<Box<Backend>> {
    let backend: Box<Backend> = match backend {
        "java" => Box::new(java::resolve(options, env)?),
        "python" => Box::new(python::resolve(options, env)?),
        _ => return Err(ErrorKind::MissingBackend.into()),
    };

    Ok(backend)
}
