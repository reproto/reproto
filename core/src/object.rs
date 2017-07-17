use errors::*;
use std::fmt;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

pub trait Object: Send + fmt::Display + fmt::Debug {
    /// Get a path to the object, if one exists.
    fn path(&self) -> Option<&Path>;

    /// Open a reader to the object.
    fn read<'a>(&'a self) -> Result<Box<Read + 'a>>;

    fn clone(&self) -> Box<Object>;
}

#[derive(Debug)]
pub struct BytesObject {
    name: String,
    bytes: Vec<u8>,
}

impl BytesObject {
    pub fn new(name: String, bytes: Vec<u8>) -> BytesObject {
        BytesObject {
            name: name,
            bytes: bytes,
        }
    }
}

impl Object for BytesObject {
    fn path(&self) -> Option<&Path> {
        None
    }

    fn read<'a>(&'a self) -> Result<Box<Read + 'a>> {
        Ok(Box::new(Cursor::new(&self.bytes)))
    }

    fn clone(&self) -> Box<Object> {
        Box::new(BytesObject {
            name: self.name.clone(),
            bytes: self.bytes.clone(),
        })
    }
}

impl fmt::Display for BytesObject {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "<{}>", self.name)
    }
}

#[derive(Debug)]
pub struct PathObject {
    path: PathBuf,
}

impl PathObject {
    pub fn new<P: AsRef<Path>>(path: P) -> PathObject {
        PathObject { path: path.as_ref().to_owned() }
    }
}

impl Object for PathObject {
    fn path(&self) -> Option<&Path> {
        Some(self.path.as_ref())
    }

    fn read(&self) -> Result<Box<Read>> {
        Ok(Box::new(File::open(&self.path)?))
    }

    fn clone(&self) -> Box<Object> {
        Box::new(PathObject { path: self.path.clone() })
    }
}

impl fmt::Display for PathObject {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.path.display())
    }
}

impl From<PathObject> for Box<Object> {
    fn from(value: PathObject) -> Box<Object> {
        Box::new(value)
    }
}
