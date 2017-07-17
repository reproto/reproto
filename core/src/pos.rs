use object::Object;
use serde;
use std::rc::Rc;
use std::result;

#[derive(Clone, Serialize, Debug)]
pub struct Pos {
    #[serde(serialize_with = "serialize_object")]
    pub object: Rc<Box<Object>>,
    pub start: usize,
    pub end: usize,
}

fn serialize_object<S>(object: &Rc<Box<Object>>, serializer: S) -> result::Result<S::Ok, S::Error>
    where S: serde::Serializer
{
    serializer.serialize_str(format!("{}", *object).as_ref())
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
