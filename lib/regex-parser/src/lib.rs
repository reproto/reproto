use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammar);

use reproto_core::errors::{Error, Result};
use reproto_core::regex::Regex;

pub fn parse(input: &str) -> Result<Regex> {
    use lalrpop_util::ParseError::*;
    use regex_lexer::Error::*;

    let lexer = regex_lexer::regex_lex(input);

    let parser = grammar::RegexParser::new();

    match parser.parse(lexer) {
        Ok(file) => Ok(file),
        Err(e) => match e {
            InvalidToken { location } => {
                Err(Error::new(format!("invalid token at char #{}", location)))
            }
            UnrecognizedToken { .. } => Err(format!("syntax error").into()),
            User { error } => match error {
                Unexpected { pos } => {
                    return Err(Error::new(format!("unexpected input at char #{}", pos)));
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
    fn test_basic_regex() {
        let regex = parse("[a-z]+").unwrap();
        println!("regex = {:?}", regex);
    }
}
