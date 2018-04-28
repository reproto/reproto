#![recursion_limit = "1000"]

extern crate lalrpop_util;
extern crate num_bigint;
extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_lexer as lexer;

#[allow(unused)]
mod parser;
mod utils;

use core::Diagnostics;
use core::errors::Result;
use std::io::Read;
use std::result;

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
pub fn parse<'input>(
    diag: &mut Diagnostics,
    input: &'input str,
) -> result::Result<ast::File<'input>, ()> {
    use self::lexer::errors::Error::*;
    use lalrpop_util::ParseError::*;

    let lexer = lexer::lex(input);
    let parser = parser::FileParser::new();

    match parser.parse(lexer) {
        Ok(file) => Ok(file),
        Err(e) => match e {
            InvalidToken { location } => {
                let span = (location, location);
                diag.err(span, "syntax error");
                Err(())
            }
            ExtraToken {
                token: (start, token, end),
            } => {
                diag.err((start, end), format!("extra token: {:?}", token));
                Err(())
            }
            UnrecognizedToken { token, expected } => {
                match token {
                    Some((start, token, end)) => {
                        diag.err(
                            (start, end),
                            format!(
                                "syntax error, got token {:?}, expected: {}",
                                token,
                                expected.join(", ")
                            ),
                        );
                    }
                    None => {
                        let m = format!("syntax error, expected: {}", expected.join(", "));
                        diag.err((0, 0), m);
                    }
                };

                return Err(());
            }
            User { error } => match error {
                UnterminatedString { start } => {
                    diag.err((start, start), "unterminated string");
                    return Err(());
                }
                UnterminatedEscape { start } => {
                    diag.err((start, start), "unterminated escape sequence");
                    return Err(());
                }
                InvalidEscape { pos, message } => {
                    diag.err((pos, pos), message);
                    return Err(());
                }
                UnterminatedCodeBlock { start } => {
                    diag.err((start, start), "unterminated code block");
                    return Err(());
                }
                InvalidNumber { pos, message } => {
                    diag.err((pos, pos), message);
                    return Err(());
                }
                Unexpected { pos } => {
                    diag.err((pos, pos), "unexpected input");
                    return Err(());
                }
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::*;
    use core::*;

    /// Check that a parsed value equals expected.
    macro_rules! assert_value_eq {
        ($expected:expr, $input:expr) => {{
            let v = parser::ValueParser::new().parse(parse($input)).unwrap();
            assert_eq!($expected, v);
        }};
    }

    macro_rules! assert_type {
        ($expected:expr, $input:expr) => {{
            let v = parser::TypeParser::new().parse(parse($input)).unwrap();
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
            .parse(parse(input))
            .expect("bad file")
    }

    fn parse_member(input: &'static str) -> TypeMember {
        parser::TypeMemberParser::new()
            .parse(parse(input))
            .expect("bad type member")
    }

    fn parse_type(input: &'static str) -> Type {
        parser::TypeParser::new()
            .parse(parse(input))
            .expect("bad type")
    }

    #[test]
    fn test_file1() {
        let input = ::std::str::from_utf8(FILE1).unwrap();
        let _ = parse_file(input);
    }

    #[test]
    fn test_array() {
        let ty = parse_type("[string]");

        if let Type::Array { inner } = ty {
            if let Type::String = *Loc::borrow(inner.as_ref()) {
                return;
            }
        }

        panic!("Expected Type::Array(Type::String)");
    }

    #[test]
    fn test_map() {
        let ty = parse_type("{string: u32}");

        // TODO: use #![feature(box_patterns)]:
        // if let Type::Map(box Type::String, box Type::Unsigned(size)) = ty {
        // }
        if let Type::Map { key, value } = ty {
            if let Type::String = *Loc::borrow(key.as_ref()) {
                if let Type::Unsigned { ref size } = *Loc::borrow(value.as_ref()) {
                    assert_eq!(32, *size);
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
            path: vec![
                Loc::new("Hello".into(), Span::empty()),
                Loc::new("World".into(), Span::empty()),
            ],
        };

        assert_type!(Type::String, "string");
        assert_type!(
            Type::Name {
                name: Loc::new(c, Span::empty())
            },
            "Hello::World"
        );
    }
}
