use errors::Result;
use ropey::Rope;
use std::fmt;
use std::fs::File;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;
use utils::{find_range, Position};
use {Encoding, RelativePathBuf, Span};

#[derive(Debug, Clone)]
pub enum Readable {
    Empty,
    Bytes(Arc<Vec<u8>>),
    Path(Arc<PathBuf>),
    Rope(Url, Rope),
    Stdin,
}

impl Readable {
    /// Open a reader for this readable.
    fn read(&self) -> Result<Box<Read>> {
        use self::Readable::*;

        let out: Box<Read> = match *self {
            Empty => Box::new(Cursor::new(&[])),
            Bytes(ref bytes) => Box::new(Cursor::new(ArcCursor(Arc::clone(&bytes)))),
            Path(ref path) => Box::new(File::open(path.as_ref())?),
            Rope(_, ref rope) => Box::new(Cursor::new(ArcCursor(Arc::new(
                rope.to_string().into_bytes(),
            )))),
            Stdin => Box::new(io::stdin()),
        };

        Ok(out)
    }
}

impl fmt::Display for Readable {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Readable::*;

        match *self {
            Empty => "empty".fmt(fmt),
            Bytes(ref bytes) => write!(fmt, "bytes:{}", bytes.len()),
            Path(ref path) => write!(fmt, "path:{}", path.display()),
            Rope(ref url, _) => write!(fmt, "rope:{}", url),
            Stdin => "stdin".fmt(fmt),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    name: Option<Arc<String>>,
    path: Option<Arc<PathBuf>>,
    readable: Readable,
}

impl Source {
    /// Create a new empty source.
    pub fn empty<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: Some(Arc::new(name.as_ref().to_string())),
            path: None,
            readable: Readable::Empty,
        }
    }

    /// Create a new empty source.
    pub fn rope<U: Into<Url>>(url: U, rope: Rope) -> Self {
        let url = url.into();

        Self {
            name: Some(Arc::new(url.to_string())),
            path: None,
            readable: Readable::Rope(url, rope),
        }
    }

    /// Create a new bytes source.
    pub fn bytes<S: AsRef<str>>(name: S, bytes: Vec<u8>) -> Self {
        Self {
            name: Some(Arc::new(name.as_ref().to_string())),
            path: None,
            readable: Readable::Bytes(Arc::new(bytes)),
        }
    }

    /// Create a new path source.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            name: None,
            path: None,
            readable: Readable::Path(Arc::new(path.as_ref().to_owned())),
        }
    }

    /// Create an source from stdin.
    pub fn stdin() -> Self {
        Self {
            name: None,
            path: None,
            readable: Readable::Stdin,
        }
    }

    /// Access the path of the source.
    pub fn path(&self) -> Option<&Path> {
        if let Some(path) = self.path.as_ref() {
            return Some(path.as_ref());
        }

        if let Readable::Path(ref path) = self.readable {
            return Some(path.as_ref());
        }

        None
    }

    /// Access the URL for this source if it is a rope.
    pub fn rope_url(&self) -> Option<&Url> {
        if let Readable::Rope(ref url, _) = self.readable {
            return Some(url);
        }

        None
    }

    /// Access the URL for this source.
    pub fn url(&self) -> Option<Url> {
        if let Readable::Rope(ref url, _) = self.readable {
            return Some(url.clone());
        }

        if let Some(path) = self.path() {
            let path = match path.canonicalize() {
                Ok(path) => path,
                Err(_) => return None,
            };

            match Url::from_file_path(path) {
                Ok(url) => return Some(url),
                Err(_) => {}
            }
        }

        None
    }

    /// Access a rope.
    pub fn as_rope(&self) -> Option<&Rope> {
        if let Readable::Rope(_, ref rope) = self.readable {
            return Some(rope);
        }

        None
    }

    /// Access a mutable rope.
    pub fn as_mut_rope(&mut self) -> Option<&mut Rope> {
        if let Readable::Rope(_, ref mut rope) = self.readable {
            return Some(rope);
        }

        None
    }

    /// Open up a readable.
    pub fn read(&self) -> Result<Box<Read>> {
        self.readable.read()
    }

    /// Create a copy of this source that has a different name.
    pub fn with_name(&self, name: String) -> Self {
        Self {
            name: Some(Arc::new(name)),
            path: self.path.as_ref().map(Arc::clone),
            readable: self.readable.clone(),
        }
    }

    pub fn span_to_range(&self, span: Span, encoding: Encoding) -> Result<(Position, Position)> {
        // ropes are stored in-memory and has custom facilities for solving this.
        /*if let Some(rope) = self.as_rope() {
            let mut start = Position::default();
            let mut end = Position::default();

            let start.line = rope.char_to_line(span.start);
            let end.line = rope.char_to_line(span.end);

            let start_line = rope.line(start.line).bytes().collect::<Vec<_>>();
            let end_line = rope.line(end.line).bytes().collect::<Vec<_>>();

            return Ok((start, end));
        }*/

        find_range(self.read()?, span, encoding)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Readable::Path(ref path) = self.readable {
            if path.is_absolute() {
                return path.display().fmt(fmt);
            }

            // platform-neutral formatting
            return RelativePathBuf::from_path(path.as_ref())
                .map_err(|_| fmt::Error)?
                .display()
                .fmt(fmt);
        }

        match self.name {
            Some(ref name) => write!(fmt, "<{} {}>", name, self.readable),
            None => write!(fmt, "<{}>", self.readable),
        }
    }
}

/// Adapt a vector in an Arc to be used in a Cursor.
struct ArcCursor(Arc<Vec<u8>>);

impl AsRef<[u8]> for ArcCursor {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
