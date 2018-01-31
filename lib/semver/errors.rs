//! Errors for this crate.

use parser;
use std::error;
use std::fmt;

/// An error type for this crate.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error<'input> {
    /// An error ocurred while parsing.
    Parser(parser::Error<'input>),
    /// Received more input than expected.
    MoreInput,
}

impl<'input> fmt::Display for Error<'input> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            Parser(ref p) => write!(fmt, "parser error: {}", p),
            MoreInput => write!(fmt, "more input"),
        }
    }
}

impl<'input> error::Error for Error<'input> {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            Parser(ref p) => p.description(),
            MoreInput => "more input",
        }
    }
}

impl<'input> From<parser::Error<'input>> for Error<'input> {
    fn from(value: parser::Error<'input>) -> Self {
        Error::Parser(value)
    }
}
