use backend::Backend;
use backend::java::fasterxml::FasterXmlBackend;
use errors::*;

pub fn resolve(backend: &str) -> Result<Box<Backend>> {
    match backend {
        "fasterxml" => Ok(Box::new(FasterXmlBackend::new())),
        _ => Err(ErrorKind::MissingBackend.into()),
    }
}
