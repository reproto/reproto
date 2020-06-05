//! Lexer used for parsing reproto manifests.

#[macro_use]
pub(crate) mod macros;
pub mod errors;
pub(crate) mod lexer;
pub(crate) mod token;

pub use self::errors::Error;
pub use self::lexer::{lex, match_keyword};
pub use self::token::{Keyword, Token};
