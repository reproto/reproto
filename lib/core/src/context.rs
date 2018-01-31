//! The context of a single execution.
//!
//! Is used to accumulate errors.
//!
//! This is preferred over results, since it permits reporting complex errors and their
//! corresponding locations.

use error_pos::ErrorPos;
use errors::{Error, ErrorKind};
use std::cell::{BorrowError, Ref, RefCell};
use std::fmt;

pub enum ContextItem {
    /// A positional error.
    ErrorPos(ErrorPos, String),
    /// A positional information string.
    InfoPos(ErrorPos, String),
}

#[derive(Default)]
pub struct Context {
    errors: RefCell<Vec<ContextItem>>,
}

/// A reporter that processes the given error for the context.
///
/// Converting the reporter into an `ErrorKind` causes it to accumulate the errors to the `Context`.
pub struct Reporter<'a> {
    ctx: &'a Context,
    errors: Vec<ContextItem>,
}

impl<'a> Reporter<'a> {
    pub fn err<P: Into<ErrorPos>, E: fmt::Display>(mut self, pos: P, error: E) -> Self {
        self.errors
            .push(ContextItem::ErrorPos(pos.into(), error.to_string()));

        self
    }

    pub fn info<P: Into<ErrorPos>, I: fmt::Display>(mut self, pos: P, info: I) -> Self {
        self.errors
            .push(ContextItem::InfoPos(pos.into(), info.to_string()));

        self
    }

    /// Close the reporter, saving any reported errors to the context.
    pub fn close(self) -> Option<Error> {
        if self.errors.is_empty() {
            return None;
        }

        let ctx = self.ctx;

        let mut errors = ctx.errors
            .try_borrow_mut()
            .expect("exclusive mutable access");

        errors.extend(self.errors);
        Some(ErrorKind::Context.into())
    }
}

impl<'a> From<Reporter<'a>> for Error {
    fn from(reporter: Reporter<'a>) -> Error {
        reporter.close();
        ErrorKind::Context.into()
    }
}

impl Context {
    /// Build a handle that can be used in conjunction with Result#map_err.
    pub fn report(&self) -> Reporter {
        Reporter {
            ctx: self,
            errors: Vec::new(),
        }
    }

    /// Iterate over all reporter errors.
    pub fn errors(&self) -> Result<Ref<Vec<ContextItem>>, BorrowError> {
        self.errors.try_borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use errors::*;
    use object::{BytesObject, Object};
    use pos::Pos;
    use std::rc::Rc;
    use std::result;
    use std::sync::Arc;

    #[test]
    fn test_handle() {
        let object = BytesObject::new("test".to_string(), Arc::new(Vec::new()));

        let pos: Pos = (Rc::new(object.clone_object()), 0usize, 0usize).into();
        let other_pos: Pos = (Rc::new(object.clone_object()), 0usize, 0usize).into();

        let ctx = Context::default();

        let result: result::Result<(), &str> = Err("nope");

        let a: Result<()> = result.map_err(|e| {
            ctx.report()
                .err(pos, e)
                .err(other_pos, "previously reported here")
                .into()
        });

        let e = a.unwrap_err();

        match e.kind() {
            &ErrorKind::Context => {}
            other => panic!("unexpected: {:?}", other),
        }

        assert_eq!(2, ctx.errors().unwrap().len());
    }
}
