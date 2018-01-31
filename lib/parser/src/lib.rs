#![recursion_limit = "1000"]

#[macro_use]
extern crate error_chain;
extern crate lalrpop_util;
extern crate num;
extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_lexer as lexer;

mod utils;
pub mod errors;
#[allow(unused)]
mod parser;

use self::errors::*;
use core::Object;
use std::io::Read;
use std::rc::Rc;

pub fn read_reader<'a, R>(mut reader: R) -> Result<String>
where
    R: AsMut<Read + 'a>,
{
    let mut content = String::new();
    reader.as_mut().read_to_string(&mut content)?;
    Ok(content)
}

pub fn parse_string<'input>(
    object: Rc<Box<Object>>,
    input: &'input str,
) -> Result<ast::File<'input>> {
    use self::ErrorKind::*;
    use self::lexer::errors::Error::*;
    use lalrpop_util::ParseError::*;

    let lexer = lexer::lex(input);

    match parser::parse_File(&object, lexer) {
        Ok(file) => Ok(file),
        Err(e) => match e {
            InvalidToken { location } => {
                let pos = (object.clone(), location, location);
                Err(Syntax(Some(pos.into()), vec![]).into())
            }
            UnrecognizedToken { token, expected } => {
                let pos = token.map(|(start, _, end)| (object.clone(), start, end));
                Err(Syntax(pos.map(Into::into), expected).into())
            }
            User { error } => match error {
                UnterminatedString { start } => {
                    let pos = (object.clone(), start, start);
                    return Err(Parse("unterminated string", pos.into()).into());
                }
                UnterminatedEscape { start } => {
                    let pos = (object.clone(), start, start);
                    return Err(Parse("unterminated escape sequence", pos.into()).into());
                }
                InvalidEscape { pos, message } => {
                    let pos = (object.clone(), pos, pos);
                    return Err(Parse(message, pos.into()).into());
                }
                UnterminatedCodeBlock { start } => {
                    let pos = (object.clone(), start, start);
                    return Err(Parse("unterminated code block", pos.into()).into());
                }
                InvalidNumber { pos, message } => {
                    let pos = (object.clone(), pos, pos);
                    return Err(Parse(message, pos.into()).into());
                }
                Unexpected { pos } => {
                    let pos = (object.clone(), pos, pos);
                    return Err(Parse("unexpected input", pos.into()).into());
                }
            },
            _ => Err("parse error".into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ast::*;
    use core::*;
    use std::rc::Rc;
    use std::sync::Arc;

    fn new_context() -> Rc<Box<Object>> {
        Rc::new(Box::new(BytesObject::new(
            String::from(""),
            Arc::new(vec![]),
        )))
    }

    /// Check that a parsed value equals expected.
    macro_rules! assert_value_eq {
        ($expected:expr, $input:expr) => {{
            let v = parser::parse_Value(&new_context(), parse($input)).unwrap();
            assert_eq!($expected, v);
        }}
    }

    macro_rules! assert_type_spec_eq {
        ($expected:expr, $input:expr) => {{
            let v = parser::parse_TypeSpec(&new_context(), parse($input)).unwrap();
            assert_eq!($expected, v);
        }}
    }

    const FILE1: &[u8] = include_bytes!("tests/file1.reproto");
    const INTERFACE1: &[u8] = include_bytes!("tests/interface1.reproto");

    fn parse(
        input: &'static str,
    ) -> Box<Iterator<Item = lexer::errors::Result<(usize, lexer::Token<'static>, usize)>>> {
        Box::new(lexer::lex(input))
    }

    fn parse_file(input: &'static str) -> File {
        parser::parse_File(&new_context(), parse(input)).unwrap()
    }

    fn parse_member(input: &'static str) -> TypeMember {
        parser::parse_TypeMember(&new_context(), parse(input)).unwrap()
    }

    fn parse_type_spec(input: &'static str) -> Type {
        parser::parse_TypeSpec(&new_context(), parse(input)).unwrap()
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
            }),
            "1.25"
        );

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: 12500.into(),
                decimal: 0,
            }),
            "1.25e4"
        );

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: (-12500).into(),
                decimal: 0,
            }),
            "-1.25e4"
        );

        assert_value_eq!(
            Value::Number(RpNumber {
                digits: (1234).into(),
                decimal: 8,
            }),
            "0.00001234"
        );
    }

    #[test]
    fn test_type_spec() {
        let c = Name::Absolute {
            prefix: None,
            parts: vec!["Hello".to_owned(), "World".to_owned()],
        };

        assert_type_spec_eq!(Type::String, "string");
        assert_type_spec_eq!(Type::Name { name: c }, "Hello::World");
    }

    #[test]
    fn test_option_decl() {
        let member = parse_member("option foo_bar_baz = true;");

        if let TypeMember::Option(option) = member {
            assert_eq!("foo_bar_baz", option.name);
            assert_eq!(Value::Boolean(true), *option.value.value());
            return;
        }

        panic!("option did not match");
    }
}
