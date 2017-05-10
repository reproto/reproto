use backend::Backend;
use backend::java;

use std::path::PathBuf;

pub struct Input {
    path: PathBuf,
}

pub enum ResolveError {
    Unknown,
}

pub fn resolve(input: &Input) -> Result<Box<Backend>, ResolveError> {
    Err(ResolveError::Unknown)
}
