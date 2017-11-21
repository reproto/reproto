use super::{Object, Pos};
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Debug)]
pub struct ErrorPos {
    pub object: Box<Object>,
    pub start: usize,
    pub end: usize,
}

impl ErrorPos {
    /// Needs explicit method because object is boxed.
    pub fn clone_error_pos(&self) -> ErrorPos {
        ErrorPos {
            object: self.object.clone_object(),
            start: self.start,
            end: self.end,
        }
    }
}

impl<T: Borrow<Pos>> From<T> for ErrorPos {
    fn from(value: T) -> ErrorPos {
        let value = value.borrow();

        ErrorPos {
            object: (**value.object).clone_object(),
            start: value.start,
            end: value.end,
        }
    }
}

impl From<(Rc<Box<Object>>, usize, usize)> for ErrorPos {
    fn from(value: (Rc<Box<Object>>, usize, usize)) -> Self {
        ErrorPos {
            object: (**value.0).clone_object(),
            start: value.1,
            end: value.2,
        }
    }
}
