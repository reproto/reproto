use super::Object;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
pub struct Pos {
    #[serde(skip)]
    pub object: Rc<Box<Object>>,
    pub start: usize,
    pub end: usize,
}

impl From<(Rc<Box<Object>>, usize, usize)> for Pos {
    fn from(value: (Rc<Box<Object>>, usize, usize)) -> Self {
        Pos {
            object: value.0,
            start: value.1,
            end: value.2,
        }
    }
}
