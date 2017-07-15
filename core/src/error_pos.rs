use object::Object;
use std::borrow::Borrow;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use super::Pos;

#[derive(Debug)]
pub struct ErrorPos {
    pub object: Arc<Mutex<Box<Object>>>,
    pub start: usize,
    pub end: usize,
}

impl<T: Borrow<Pos>> From<T> for ErrorPos {
    fn from(value: T) -> ErrorPos {
        let value = value.borrow();

        ErrorPos {
            object: value.0.clone(),
            start: value.1,
            end: value.2,
        }
    }
}
