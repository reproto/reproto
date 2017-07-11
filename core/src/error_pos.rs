use std::borrow::Borrow;
use std::path::PathBuf;
use super::Pos;

#[derive(Debug)]
pub struct ErrorPos {
    pub path: PathBuf,
    pub start: usize,
    pub end: usize,
}

impl<T: Borrow<Pos>> From<T> for ErrorPos {
    fn from(value: T) -> ErrorPos {
        let value = value.borrow();

        ErrorPos {
            path: value.0.as_ref().to_owned(),
            start: value.1,
            end: value.2,
        }
    }
}
