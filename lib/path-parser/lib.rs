extern crate lalrpop_util;
extern crate reproto_ast as ast;
extern crate reproto_path_lexer as path_lexer;

mod errors;
mod parser;

use ast::PathSpec;
pub use self::errors::Error;

pub fn parse(input: &str) -> Result<PathSpec, Error> {
    use lalrpop_util::ParseError::*;
    use self::Error::*;
    use self::path_lexer::Error::*;

    let lexer = path_lexer::path_lex(input);

    match parser::parse_path(lexer) {
        Ok(file) => Ok(file),
        Err(e) => {
            match e {
                InvalidToken { location } => {
                    Err(Syntax(Some((location, location)), vec![]).into())
                }
                UnrecognizedToken { token, expected } => {
                    let pos = token.map(|(start, _, end)| (start, end));
                    Err(Syntax(pos, expected).into())
                }
                User { error } => {
                    match error {
                        Unexpected { pos } => {
                            return Err(Parse(Some((pos, pos)), "unexpected input"));
                        }
                    }
                }
                _ => Err(Parse(None, "parse error")),
            }
        }
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
