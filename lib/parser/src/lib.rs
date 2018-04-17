#![recursion_limit = "1000"]

extern crate lalrpop_util;
extern crate num_bigint;
extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_lexer as lexer;

#[allow(unused)]
mod parser;
mod utils;

use core::errors::*;
use core::Source;
use std::io::Read;
use std::sync::Arc;

/// Read the full contents of the given reader as a string.
pub fn read_to_string<'a, R>(mut reader: R) -> Result<String>
where
    R: AsMut<Read + 'a>,
{
    let mut content = String::new();
    reader.as_mut().read_to_string(&mut content)?;
    Ok(content)
}

/// Parse the given object.
pub fn parse<'input>(object: Arc<Source>, input: &'input str) -> Result<ast::File<'input>> {
    use self::lexer::errors::Error::*;
    use lalrpop_util::ParseError::*;

    let lexer = lexer::lex(input);
    let parser = parser::FileParser::new();

    match parser.parse(&object, lexer) {
        Ok(file) => Ok(file),
        Err(e) => match e {
            InvalidToken { location } => {
                let span = (object.clone(), location, location);
                Err(Error::new("syntax error").with_span(span))
            }
            UnrecognizedToken { token, expected } => {
                let e = if let Some((start, token, end)) = token {
                    let span = (object.clone(), start, end);
                    Error::new(format!(
                        "syntax error, got token {:?}, expected: {}",
                        token,
                        expected.join(", ")
                    )).with_span(span)
                } else {
                    Error::new(format!("syntax error, expected: {}", expected.join(", ")))
                };

                Err(e)
            }
            User { error } => match error {
                UnterminatedString { start } => {
                    let span = (object.clone(), start, start);
                    return Err(Error::new("unterminated string").with_span(span));
                }
                UnterminatedEscape { start } => {
                    let span = (object.clone(), start, start);
                    return Err(Error::new("unterminated escape sequence").with_span(span));
                }
                InvalidEscape { pos, message } => {
                    let span = (object.clone(), pos, pos);
                    return Err(Error::new(message).with_span(span));
                }
                UnterminatedCodeBlock { start } => {
                    let span = (object.clone(), start, start);
                    return Err(Error::new("unterminated code block").with_span(span));
                }
                InvalidNumber { pos, message } => {
                    let span = (object.clone(), pos, pos);
                    return Err(Error::new(message).with_span(span));
                }
                Unexpected { pos } => {
                    let span = (object.clone(), pos, pos);
                    return Err(Error::new("unexpected input").with_span(span));
                }
            },
            _ => Err("Parse error".into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::*;
    use core::*;
    use std::sync::Arc;

    fn new_context() -> Arc<Source> {
        Arc::new(Source::bytes("test", vec![]))
    }

    /// Check that a parsed value equals expected.
    macro_rules! assert_value_eq {
        ($expected:expr, $input:expr) => {{
            let v = parser::ValueParser::new()
                .parse(&new_context(), parse($input))
                .unwrap();
            assert_eq!($expected, v);
        }};
    }

    macro_rules! assert_type_spec_eq {
        ($expected:expr, $input:expr) => {{
            let v = parser::TypeSpecParser::new()
                .parse(&new_context(), parse($input))
                .unwrap();
            assert_eq!($expected, v);
        }};
    }

    const FILE1: &[u8] = include_bytes!("tests/file1.reproto");
    const INTERFACE1: &[u8] = include_bytes!("tests/interface1.reproto");

    fn parse(
        input: &'static str,
    ) -> Box<Iterator<Item = lexer::errors::Result<(usize, lexer::Token<'static>, usize)>>> {
        Box::new(lexer::lex(input))
    }

    fn parse_file(input: &'static str) -> File {
        parser::FileParser::new()
            .parse(&new_context(), parse(input))
            .unwrap()
    }

    fn parse_member(input: &'static str) -> TypeMember {
        parser::TypeMemberParser::new()
            .parse(&new_context(), parse(input))
            .unwrap()
    }

    fn parse_type_spec(input: &'static str) -> Type {
        parser::TypeSpecParser::new()
            .parse(&new_context(), parse(input))
            .unwrap()
    }

    #[test]
    fn test_file1() {
        let input = ::std::str::from_utf8(FILE1).unwrap();
        let _ = parse_file(input);
    }

    #[test]
    fn test_array() {
        let ty = parse_type_spec("[string]");

        if let Type::Array { inner } = ty {
            if let Type::String = *inner {
                return;
            }
        }

        panic!("Expected Type::Array(Type::String)");
    }

    #[test]
    fn test_map() {
        let ty = parse_type_spec("{string: u32}");

        // TODO: use #![feature(box_patterns)]:
        // if let Type::Map(box Type::String, box Type::Unsigned(size)) = ty {
        // }
        if let Type::Map { key, value } = ty {
            if let Type::String = *key {
                if let Type::Unsigned { size } = *value {
                    assert_eq!(32, size);
                    return;
                }
            }
        }

        panic!("Expected Type::Array(Type::String)");
    }

    #[test]
    fn test_block_comment() {
        parse("/* hello \n world */");
    }

    #[test]
    fn test_line_comment() {
        parse("// hello world\n");
    }

    #[test]
    fn test_code() {
        parse_member("java{{\na { b { c } d } e\n}}");
    }

    #[test]
    fn test_interface() {
        let input = ::std::str::from_utf8(INTERFACE1).unwrap();
        let file = parse_file(input);
        assert_eq!(1, file.decls.len());
    }

    #[test]
    fn test_strings() {
        assert_value_eq!(Value::String("foo\nbar".to_owned()), "\"foo\\nbar\"");
    }

    #[test]
    fn test_numbers() {
        assert_value_eq!(Value::Number(1.into()), "1");

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: 125.into(),
                decimal: 2,
            },),
            "1.25"
        );

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: 12500.into(),
                decimal: 0,
            },),
            "1.25e4"
        );

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: (-12500).into(),
                decimal: 0,
            },),
            "-1.25e4"
        );

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: (1234).into(),
                decimal: 8,
            },),
            "0.00001234"
        );
    }

    #[test]
    fn test_type_spec() {
        let c = Name::Absolute {
            prefix: None,
            parts: Loc::new(
                vec!["Hello".to_string(), "World".to_string()].into(),
                Span::empty(),
            ),
        };

        assert_type_spec_eq!(Type::String, "string");
        assert_type_spec_eq!(Type::Name { name: c }, "Hello::World");
    }
}
