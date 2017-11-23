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

pub enum ContextError {
    /// A positional error.
    Pos(ErrorPos, String),
}

pub struct Context {
    errors: RefCell<Vec<ContextError>>,
}

/// A reporter that processes the given error for the context.
///
/// Converting the reporter into an ErrorKind causes it to accumulate the errors to the `Context`.
pub struct Reporter<'a> {
    ctx: &'a Context,
    errors: Vec<ContextError>,
}

impl<'a> Reporter<'a> {
    pub fn err<P: Into<ErrorPos>, E: fmt::Display>(mut self, pos: P, error: E) -> Self {
        self.errors.push(
            ContextError::Pos(pos.into(), error.to_string()),
        );
        self
    }
}

impl<'a> From<Reporter<'a>> for Error {
    fn from(reporter: Reporter<'a>) -> Error {
        let ctx = reporter.ctx;
        let mut errors = ctx.errors.try_borrow_mut().expect(
            "exclusive mutable access",
        );
        errors.extend(reporter.errors);
        ErrorKind::Context.into()
    }
}

impl Context {
    pub fn new() -> Context {
        Context { errors: RefCell::new(Vec::new()) }
    }

    /// Build a handle that can be used in conjunction with Result#map_err.
    pub fn report(&self) -> Reporter {
        Reporter {
            ctx: self,
            errors: Vec::new(),
        }
    }

    /// Iterate over all reporter errors.
    pub fn errors(&self) -> Result<Ref<Vec<ContextError>>, BorrowError> {
        self.errors.try_borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use errors::ErrorKind;
    use object::{BytesObject, Object};
    use pos::Pos;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn test_handle() {
        let object = BytesObject::new("test".to_string(), Arc::new(Vec::new()));

        let pos: Pos = (Rc::new(object.clone_object()), 0usize, 0usize).into();
        let other_pos: Pos = (Rc::new(object.clone_object()), 0usize, 0usize).into();

        let mut ctx = Context::new();

        let result: Result<(), &str> = Err("nope");

        let a = result.map_err(|e| {
            ctx.report()
                .err(pos, e)
                .err(other_pos, "previously reported here")
                .into()
        });

        match a {
            Err(ErrorKind::Context) => {}
            other => panic!("unexpected: {:?}", other),
        }

        assert_eq!(2, ctx.errors().count());
    }
}
