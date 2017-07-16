use object::Object;
use serde;
use std::result;
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Debug)]
pub struct Pos {
    #[serde(serialize_with = "serialize_object")]
    pub object: Arc<Mutex<Box<Object>>>,
    pub start: usize,
    pub end: usize,
}

fn serialize_object<S>(object: &Arc<Mutex<Box<Object>>>,
                       serializer: S)
                       -> result::Result<S::Ok, S::Error>
    where S: serde::Serializer
{
    let object = object.lock().map_err(|_| serde::ser::Error::custom("failed to lock"))?;
    serializer.serialize_str(format!("{}", object).as_ref())
}

impl From<(Arc<Mutex<Box<Object>>>, usize, usize)> for Pos {
    fn from(value: (Arc<Mutex<Box<Object>>>, usize, usize)) -> Self {
        Pos {
            object: value.0,
            start: value.1,
            end: value.2,
        }
    }
}
