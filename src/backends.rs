use backend::Backend;
use backend::java::fasterxml::FasterXmlBackend;
use backend::python::plain::PlainPythonBackend;
use errors::*;

pub fn resolve(backend: &str) -> Result<Box<Backend>> {
    match backend {
        "java" | "java/fasterxml" => Ok(Box::new(FasterXmlBackend::new())),
        "python" | "python/plain" => Ok(Box::new(PlainPythonBackend::new())),
        _ => Err(ErrorKind::MissingBackend.into()),
    }
}
