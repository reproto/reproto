//! The primary abstraction to indicate a "source" of data.
//!
//! Sources are primarily available to be read through `Source::read`, but also support auxiliary
//! functions like finding the line of a given span `Source::find_range` or printing pretty
//! diagnostics.

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
    /// No source, typically used for tests.
    ///
    /// When read will successfully return an empty Read.
    Empty,
    /// Bytes in-memory containing the source.
    Bytes(Arc<Vec<u8>>),
    /// Path to a file containing the source.
    Path(Arc<PathBuf>),
    /// In-memory data structure that supports non-linear editing efficiently.
    Rope(Url, Rope),
    /// Read from Stdin, typically can only be read once (so make it count!).
    Stdin,
}

impl Readable {
    /// Open a reader for this readable.
    ///
    /// Note: It is not guaranteed that it is possible to open the same source multiple times.
    fn read(&self) -> Result<Box<Read>> {
        use self::Readable::*;

        let out: Box<Read> = match *self {
            Empty => Box::new(Cursor::new(&[])),
            Bytes(ref bytes) => Box::new(Cursor::new(ArcCursor(Arc::clone(&bytes)))),
            Path(ref path) => Box::new(
                File::open(path.as_ref())
                    .map_err(|e| format!("failed to open path: {}: {}", path.display(), e))?,
            ),
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
    /// The name of the source.
    name: Option<Arc<String>>,
    /// A filesystem path to the source, if the source has one.
    ///
    /// This path might be used to modify the source, or to print where a specific error originated
    /// from for diagnostics.
    path: Option<Arc<PathBuf>>,
    /// If this source is read-only.
    ///
    /// If this is set, a source may _not_ be modified.
    ///
    /// This is commonly set for objects from a repository to avoid modifying the files in the
    /// repository.
    pub read_only: bool,
    /// The readable accessor to the source.
    readable: Readable,
}

impl Source {
    /// Create a new empty source.
    pub fn empty<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: Some(Arc::new(name.as_ref().to_string())),
            path: None,
            read_only: true,
            readable: Readable::Empty,
        }
    }

    /// Create a new empty source.
    ///
    /// These are _not_ read-only by default. It is expected that ropes are in-memory
    /// representations of files in the filesystem.
    pub fn rope<U: Into<Url>>(url: U, rope: Rope) -> Self {
        let url = url.into();

        Self {
            name: Some(Arc::new(url.to_string())),
            path: None,
            read_only: false,
            readable: Readable::Rope(url, rope),
        }
    }

    /// Create a new bytes source.
    ///
    /// Byte sources are read-only by default.
    /// This can be turned off if this is an in-memory representation of something in the
    /// filesystem, in which case `path` should also be set to indicate where.
    pub fn bytes<S: AsRef<str>>(name: S, bytes: Vec<u8>) -> Self {
        Self {
            name: Some(Arc::new(name.as_ref().to_string())),
            path: None,
            read_only: true,
            readable: Readable::Bytes(Arc::new(bytes)),
        }
    }

    /// Create a new path source.
    ///
    /// Path sources are _not_ read-only by default.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            name: None,
            path: None,
            read_only: false,
            readable: Readable::Path(Arc::new(path.as_ref().to_owned())),
        }
    }

    /// Create an source from stdin.
    ///
    /// Stdin sources are _always_ read-only.
    pub fn stdin() -> Self {
        Self {
            name: None,
            path: None,
            read_only: true,
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

    /// Access the URL for this source, but only if it is a rope.
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

            if let Ok(url) = Url::parse(&format!("file://{}", path.display())) {
                return Some(url);
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
            read_only: self.read_only,
            readable: self.readable.clone(),
        }
    }

    /// Modify the source to set if it is read only or not.
    pub fn with_read_only(self, read_only: bool) -> Self {
        Self { read_only, ..self }
    }

    pub fn span_to_range(&self, span: Span, encoding: Encoding) -> Result<(Position, Position)> {
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
