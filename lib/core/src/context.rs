//! The context of a single execution.
//!
//! Is used to accumulate errors.
//!
//! This is preferred over results, since it permits reporting complex errors and their
//! corresponding locations.

use errors::{Error, Result};
use flavored::RpName;
use std::cell::{BorrowError, Ref, RefCell};
use std::fmt;
use std::path::Path;
use std::rc::Rc;
use std::result;
use {Filesystem, Handle, Pos};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SymbolKind {
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "tuple")]
    Tuple,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "service")]
    Service,
}

#[derive(Debug)]
pub enum ContextItem {
    /// A positional error.
    Error(Pos, String),
    /// A positional information string.
    Info(Pos, String),
    /// A symbol that was encountered, and its location.
    Symbol {
        kind: SymbolKind,
        pos: Pos,
        name: RpName,
    },
}

#[derive(Clone)]
/// Context for a single reproto run.
pub struct Context {
    /// Filesystem abstraction.
    filesystem: Rc<Box<Filesystem>>,
    /// Collected context items.
    items: Rc<RefCell<Vec<ContextItem>>>,
}

/// A reporter that processes the given error for the context.
///
/// Converting the reporter into an `ErrorKind` causes it to accumulate the errors to the `Context`.
pub struct Reporter<'a> {
    ctx: &'a Context,
    items: Vec<ContextItem>,
}

impl<'a> Reporter<'a> {
    pub fn err<P: Into<Pos>, E: fmt::Display>(&mut self, pos: P, error: E) {
        self.items
            .push(ContextItem::Error(pos.into(), error.to_string()));
    }

    pub fn info<P: Into<Pos>, I: fmt::Display>(&mut self, pos: P, info: I) {
        self.items
            .push(ContextItem::Info(pos.into(), info.to_string()));
    }

    /// Close this reporter and return an error if it has errors.
    ///
    /// This will cause the reporter to be dropped, which will report all sub-items to the parent
    /// context.
    pub fn close(self) -> Option<Error> {
        if !self.has_errors() {
            return None;
        }

        Some(Error::new("Error in Context"))
    }

    /// Check if reporter has any errors as sub-items.
    fn has_errors(&self) -> bool {
        self.items.iter().any(|item| match *item {
            ContextItem::Error(_, _) => true,
            _ => false,
        })
    }
}

impl<'a> Drop for Reporter<'a> {
    fn drop(&mut self) {
        self.ctx
            .items
            .try_borrow_mut()
            .expect("exclusive mutable access")
            .extend(self.items.drain(..));
    }
}

impl<'a> From<Reporter<'a>> for Error {
    fn from(_: Reporter<'a>) -> Error {
        Error::new("Error in Context")
    }
}

impl Context {
    /// Create a new context with the given filesystem.
    pub fn new(filesystem: Box<Filesystem>) -> Context {
        Context {
            filesystem: Rc::new(filesystem),
            items: Rc::new(RefCell::new(vec![])),
        }
    }

    /// Modify the existing context with a reference to the given errors.
    pub fn with_items(self, items: Rc<RefCell<Vec<ContextItem>>>) -> Context {
        Context { items, ..self }
    }

    /// Map the existing filesystem and return a new context with the new filesystem.
    pub fn map_filesystem<F>(self, map: F) -> Self
    where
        F: FnOnce(Rc<Box<Filesystem>>) -> Box<Filesystem>,
    {
        Context {
            filesystem: Rc::new(map(self.filesystem)),
            ..self
        }
    }

    /// Retrieve the filesystem abstraction.
    pub fn filesystem(&self, root: Option<&Path>) -> Result<Box<Handle>> {
        self.filesystem.open_root(root)
    }

    /// Build a handle that can be used in conjunction with Result#map_err.
    pub fn report(&self) -> Reporter {
        Reporter {
            ctx: self,
            items: Vec::new(),
        }
    }

    /// Register a symbol.
    pub fn symbol<P: Into<Pos>>(&self, kind: SymbolKind, pos: P, name: &RpName) -> Result<()> {
        self.items.try_borrow_mut()?.push(ContextItem::Symbol {
            kind,
            pos: pos.into(),
            name: name.clone(),
        });
        Ok(())
    }

    /// Iterate over all reporter items.
    pub fn items(&self) -> result::Result<Ref<Vec<ContextItem>>, BorrowError> {
        self.items.try_borrow()
    }

    /// Check if reporter is empty.
    pub fn has_errors(&self) -> Result<bool> {
        Ok(self.items.try_borrow()?.iter().any(|item| match *item {
            ContextItem::Error(_, _) => true,
            _ => false,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::CapturingFilesystem;
    use pos::Pos;
    use source::Source;
    use std::result;
    use std::sync::Arc;

    #[test]
    fn test_handle() {
        let source = Source::bytes("test", Vec::new());

        let pos: Pos = (Arc::new(source.clone()), 0usize, 0usize).into();
        let other_pos: Pos = (Arc::new(source.clone()), 0usize, 0usize).into();

        let ctx = Context::new(Box::new(CapturingFilesystem::new()));

        let result: result::Result<(), &str> = Err("nope");

        let a: Result<()> = result.map_err(|e| {
            let mut r = ctx.report();
            r.err(pos, e);
            r.err(other_pos, "previously reported here");
            r.into()
        });

        let e = a.unwrap_err();

        match e {
            ref e if e.message() == "Error in Context" => {}
            ref other => panic!("unexpected: {:?}", other),
        }

        assert_eq!(2, ctx.items().unwrap().len());
    }
}
