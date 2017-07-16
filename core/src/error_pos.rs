use object::Object;
use std::borrow::Borrow;
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
            object: value.object.clone(),
            start: value.start,
            end: value.end,
        }
    }
}

impl From<(Arc<Mutex<Box<Object>>>, usize, usize)> for ErrorPos {
    fn from(value: (Arc<Mutex<Box<Object>>>, usize, usize)) -> Self {
        ErrorPos {
            object: value.0,
            start: value.1,
            end: value.2,
        }
    }
}
