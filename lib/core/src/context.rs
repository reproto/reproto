//! The context of a single execution.
//!
//! Is used to accumulate errors.
//!
//! This is preferred over results, since it permits reporting complex errors and their
//! corresponding locations.

use errors::Result;
use std::cell::{BorrowError, Ref, RefCell};
use std::path::Path;
use std::rc::Rc;
use std::result;
use {Diagnostics, Filesystem, Handle};

#[derive(Debug)]
pub enum ContextItem {
    /// An emitted diagnostics.
    Diagnostics { diagnostics: Diagnostics },
}

#[derive(Clone)]
/// Context for a single reproto run.
pub struct Context {
    /// Filesystem abstraction.
    filesystem: Rc<Box<Filesystem>>,
    /// Collected context items.
    items: Rc<RefCell<Vec<ContextItem>>>,
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

    /// Add the given diagnostics to this context.
    pub fn diagnostics(&self, diagnostics: Diagnostics) -> Result<()> {
        self.items
            .try_borrow_mut()
            .map_err(|_| "no mutable access to context")?
            .push(ContextItem::Diagnostics { diagnostics });

        Ok(())
    }

    /// Iterate over all reporter items.
    pub fn items(&self) -> result::Result<Ref<Vec<ContextItem>>, BorrowError> {
        self.items.try_borrow()
    }

    /// Check if reporter is empty.
    pub fn has_diagnostics(&self) -> Result<bool> {
        Ok(self.items
            .try_borrow()
            .map_err(|_| "immutable access to context")?
            .iter()
            .any(|item| match *item {
                ContextItem::Diagnostics { ref diagnostics } => diagnostics.has_errors(),
            }))
    }

    /// Clear the context of any items.
    pub fn clear(&self) -> Result<()> {
        self.items
            .try_borrow_mut()
            .map_err(|_| "no mutable access to context")?
            .clear();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {CapturingFilesystem, Diagnostics, Source, Span};

    #[test]
    fn test_handle() {
        let ctx = Context::new(Box::new(CapturingFilesystem::new()));

        let mut diag = Diagnostics::new(Source::empty("test"));
        diag.err(Span::empty(), "nope");
        diag.err(Span::empty(), "previously reported here");
        ctx.diagnostics(diag).expect("bad diagnostic");

        assert_eq!(1, ctx.items().unwrap().len());
    }
}
