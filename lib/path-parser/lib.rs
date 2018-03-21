extern crate lalrpop_util;
extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_path_lexer as path_lexer;

mod parser;

use ast::PathSpec;
use core::errors::{Error, Result};

pub fn parse(input: &str) -> Result<PathSpec> {
    use self::path_lexer::Error::*;
    use lalrpop_util::ParseError::*;

    let lexer = path_lexer::path_lex(input);

    match parser::parse_path(lexer) {
        Ok(file) => Ok(file),
        Err(e) => match e {
            InvalidToken { location } => {
                Err(Error::new(format!("Invalid token at char #{}", location)))
            }
            UnrecognizedToken { token, expected } => Err(Error::new(format!(
                "Syntax error, got: {:?}, expected: {:?}",
                token, expected
            ))),
            User { error } => match error {
                Unexpected { pos } => {
                    return Err(Error::new(format!("Unexpected input at char #{}", pos)));
                }
            },
            _ => Err(Error::new("Parse error")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_path() {
        let spec = parse("/foo\\//{bar}_baz\\{\\}").unwrap();
        println!("spec = {:?}", spec);
    }
}
