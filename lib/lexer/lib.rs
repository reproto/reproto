//! Lexer used for parsing reproto manifests.

extern crate num_bigint;
extern crate num_traits;
extern crate reproto_core as core;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod token;
pub(crate) mod lexer;
pub mod errors;

pub use self::errors::Error;
pub use self::lexer::{lex, match_keyword};
pub use self::token::Token;
