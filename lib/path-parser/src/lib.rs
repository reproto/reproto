lalrpop_mod!(grammar);

use ast::PathSpec;
use reproto_core::errors::{Error, Result};

use lalrpop_util::lalrpop_mod;

pub fn parse(input: &str) -> Result<PathSpec> {
    use lalrpop_util::ParseError::*;
    use path_lexer::Error::*;

    let lexer = path_lexer::path_lex(input);

    let parser = grammar::PathParser::new();

    match parser.parse(lexer) {
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
