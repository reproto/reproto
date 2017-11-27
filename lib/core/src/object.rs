use errors::*;
use std::fmt;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub trait Object: Send + fmt::Display + fmt::Debug {
    /// Get a path to the object, if one exists.
    fn path(&self) -> Option<&Path>;

    /// Open a reader to the object.
    fn read<'a>(&'a self) -> Result<Box<Read + 'a>>;

    /// Lightweight cloning of this object.
    fn clone_object(&self) -> Box<Object>;

    /// Convert the current object with the given name.
    fn with_name(&self, name: String) -> Box<Object>;
}

#[derive(Debug)]
pub struct BytesObject {
    name: Arc<String>,
    bytes: Arc<Vec<u8>>,
}

impl BytesObject {
    pub fn new(name: String, bytes: Arc<Vec<u8>>) -> BytesObject {
        BytesObject {
            name: Arc::new(name),
            bytes: bytes,
        }
    }
}

impl Object for BytesObject {
    fn path(&self) -> Option<&Path> {
        None
    }

    fn read<'a>(&'a self) -> Result<Box<Read + 'a>> {
        Ok(Box::new(Cursor::new(self.bytes.as_ref())))
    }

    fn clone_object(&self) -> Box<Object> {
        Box::new(BytesObject {
            name: Arc::clone(&self.name),
            bytes: Arc::clone(&self.bytes),
        })
    }

    fn with_name(&self, name: String) -> Box<Object> {
        Box::new(BytesObject {
            name: Arc::new(name),
            bytes: Arc::clone(&self.bytes),
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
    name: Option<Arc<String>>,
    path: Arc<PathBuf>,
}

impl PathObject {
    pub fn new<P: AsRef<Path>>(name: Option<String>, path: P) -> PathObject {
        PathObject {
            name: name.map(Arc::new),
            path: Arc::new(path.as_ref().to_owned()),
        }
    }
}

impl Object for PathObject {
    fn path(&self) -> Option<&Path> {
        Some(self.path.as_ref())
    }

    fn read(&self) -> Result<Box<Read>> {
        Ok(Box::new(File::open(self.path.as_path())?))
    }

    fn clone_object(&self) -> Box<Object> {
        Box::new(PathObject {
            name: self.name.clone(),
            path: Arc::clone(&self.path),
        })
    }

    fn with_name(&self, name: String) -> Box<Object> {
        Box::new(PathObject {
            name: Some(Arc::new(name)),
            path: Arc::clone(&self.path),
        })
    }
}

impl fmt::Display for PathObject {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(formatter, "{}", name)
        } else {
            write!(formatter, "{}", self.path.display())
        }
    }
}

impl From<PathObject> for Box<Object> {
    fn from(value: PathObject) -> Box<Object> {
        Box::new(value)
    }
}
