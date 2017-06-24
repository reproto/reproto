#![recursion_limit = "1000"]

#[macro_use]
extern crate error_chain;
extern crate lalrpop_util;
extern crate reproto_core;
extern crate num;
extern crate linked_hash_map;

pub mod ast;
pub mod errors;
pub mod lexer;
#[allow(unused)]
mod parser;
mod token;
mod utils;

use lalrpop_util::ParseError;
use self::errors::*;
use std::fs;
use std::io::{Read};
use std::path::Path;
use std::rc::Rc;

const NL: u8 = '\n' as u8;

pub fn find_line(path: &Path, pos: (usize, usize)) -> Result<(String, usize, (usize, usize))> {
    let file = fs::File::open(path)?;

    let mut line = 0usize;
    let mut current = 0usize;
    let mut buffer: Vec<u8> = Vec::new();

    let start = pos.0;
    let end = pos.1;

    let mut it = file.bytes().peekable();
    let mut read = 0usize;

    while let Some(b) = it.next() {
        let b = b?;
        read += 1;

        match b {
            NL => {},
            _ => {
                buffer.push(b);
                continue;
            }
        }

        let start_of_line = current;
        current += read;

        if current >= start {
            let buffer = String::from_utf8(buffer)?;
            let end = ::std::cmp::min(end, current);
            let range = (start - start_of_line, end - start_of_line);
            return Ok((buffer, line, range));
        }

        read = 0usize;
        line += 1;
        buffer.clear();
    }

    Err("bad file position".into())
}

pub fn read_file(path: &Path) -> Result<String> {
    let mut f = fs::File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn parse_file<'input>(path: &'input Path, input: &'input str) -> Result<ast::File<'input>> {
    use self::ErrorKind::*;

    let path = Rc::new(path.to_owned());
    let lexer = lexer::lex(input);

    match parser::parse_File(&path, lexer) {
        Ok(file) => Ok(file),
        Err(e) => {
            match e {
                ParseError::InvalidToken { location } => {
                    let pos = (path.clone(), location, location);
                    Err(Syntax(Some(pos.into()), vec![]).into())
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let pos = token.map(|(start, _, end)| (path.clone(), start, end));
                    Err(Syntax(pos.map(Into::into), expected).into())
                }
                ParseError::User { error } => {
                    match error {
                        token::Error::UnterminatedString { start } => {
                            let pos = (path.clone(), start, start);
                            return Err(Parse("unterminated string", pos.into()).into());
                        }
                        token::Error::UnterminatedEscape { start } => {
                            let pos = (path.clone(), start, start);
                            return Err(Parse("unterminated escape sequence", pos.into()).into());
                        }
                        token::Error::InvalidEscape { pos, message } => {
                            let pos = (path.clone(), pos, pos);
                            return Err(Parse(message, pos.into()).into());
                        }
                        token::Error::UnterminatedCodeBlock { start } => {
                            let pos = (path.clone(), start, start);
                            return Err(Parse("unterminated code block", pos.into()).into());
                        }
                        token::Error::InvalidNumber { pos, message } => {
                            let pos = (path.clone(), pos, pos);
                            return Err(Parse(message, pos.into()).into());
                        }
                        token::Error::Unexpected { pos } => {
                            let pos = (path.clone(), pos, pos);
                            return Err(Parse("unexpected input", pos.into()).into());
                        }
                        token::Error::InvalidVersion { start, end } => {
                            let pos = (path.clone(), start, end);
                            return Err(Parse("invalid version", pos.into()).into());
                        }
                        token::Error::InvalidVersionReq { start, end } => {
                            let pos = (path.clone(), start, end);
                            return Err(Parse("invalid version requirement", pos.into()).into());
                        }
                    }
                }
                _ => Err("parse error".into()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ast::*;
    use std::path::PathBuf;

    /// Check that a parsed value equals expected.
    macro_rules! assert_value_eq {
        ($expected:expr, $input:expr) => {{
            let path = Rc::new(PathBuf::from(""));
            let v = parser::parse_Value(&path, parse($input)).unwrap();
            assert_eq!($expected, v);
        }}
    }

    macro_rules! assert_type_spec_eq {
        ($expected:expr, $input:expr) => {{
            let path = Rc::new(PathBuf::from(""));
            let v = parser::parse_TypeSpec(&path, parse($input)).unwrap();
            assert_eq!($expected, v);
        }}
    }

    const FILE1: &[u8] = include_bytes!("tests/file1.reproto");
    const INTERFACE1: &[u8] = include_bytes!("tests/interface1.reproto");

    fn parse(input: &'static str) -> Box<Iterator<Item = token::Result<(usize, token::Token<'static>, usize)>>> {
        Box::new(lexer::lex(input))
    }

    fn parse_file(input: &'static str) -> File {
        let path = Rc::new(PathBuf::from(""));
        parser::parse_File(&path, parse(input)).unwrap()
    }

    fn parse_member(input: &'static str) -> Member {
        let path = Rc::new(PathBuf::from(""));
        parser::parse_Member(&path, parse(input)).unwrap()
    }

    fn parse_type_spec(input: &'static str) -> RpType {
        let path = Rc::new(PathBuf::from(""));
        parser::parse_TypeSpec(&path, parse(input)).unwrap()
    }

    #[test]
    fn test_file1() {
        let input = ::std::str::from_utf8(FILE1).unwrap();
        let file = parse_file(input);

        let package = RpPackage::new(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]);

        assert_eq!(package, file.package_decl.package);
        assert_eq!(4, file.decls.len());
    }

    #[test]
    fn test_array() {
        let ty = parse_type_spec("[string]");

        if let RpType::Array { inner } = ty {
            if let RpType::String = *inner {
                return;
            }
        }

        panic!("Expected Type::Array(Type::String)");
    }

    #[test]
    fn test_map() {
        let ty = parse_type_spec("{string: unsigned/123}");

        // TODO: use #![feature(box_patterns)]:
        // if let Type::Map(box Type::String, box Type::Unsigned(size)) = ty {
        // }
        if let RpType::Map { key, value } = ty {
            if let RpType::String = *key {
                if let RpType::Unsigned { size } = *value {
                    assert_eq!(Some(123usize), size);
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
    fn test_instance() {
        let path = Rc::new(PathBuf::from(""));

        let c = RpName {
            prefix: None,
            parts: vec!["Foo".to_owned(), "Bar".to_owned()],
        };

        let field = FieldInit {
            name: RpLoc::new("hello", (path.clone(), 8, 13)),
            value: RpLoc::new(Value::Number(12.into()), (path.clone(), 15, 17)),
        };

        let field = RpLoc::new(field, (path.clone(), 8, 17));

        let instance = Instance {
            name: c,
            arguments: RpLoc::new(vec![field], (path.clone(), 8, 17)),
        };

        assert_value_eq!(Value::Instance(RpLoc::new(instance, (path.clone(), 0, 18))),
                         "Foo.Bar(hello: 12)");
    }

    #[test]
    fn test_strings() {
        assert_value_eq!(Value::String("foo\nbar".to_owned()), "\"foo\\nbar\"");
    }

    #[test]
    fn test_numbers() {
        assert_value_eq!(Value::Number(1.into()), "1");

        assert_value_eq!(Value::Number(RpNumber {
                             digits: 125.into(),
                             decimal: 2,
                         }),
                         "1.25");

        assert_value_eq!(Value::Number(RpNumber {
                             digits: 12500.into(),
                             decimal: 0,
                         }),
                         "1.25e4");

        assert_value_eq!(Value::Number(RpNumber {
                             digits: (-12500).into(),
                             decimal: 0,
                         }),
                         "-1.25e4");

        assert_value_eq!(Value::Number(RpNumber {
                             digits: (1234).into(),
                             decimal: 8,
                         }),
                         "0.00001234");
    }

    #[test]
    fn test_type_spec() {
        let c = RpName {
            prefix: None,
            parts: vec!["Hello".to_owned(), "World".to_owned()],
        };

        assert_type_spec_eq!(RpType::String, "string");
        assert_type_spec_eq!(RpType::Name { name: c }, "Hello.World");
    }

    #[test]
    fn test_option_decl() {
        let member = parse_member("foo_bar_baz true, foo, \"bar\", 12;");

        if let Member::Option(option) = member {
            assert_eq!("foo_bar_baz", option.name);
            assert_eq!(4, option.values.len());

            assert_eq!(Value::Boolean(true), *option.values[0].as_ref());
            assert_eq!(Value::Identifier("foo".to_owned()), *option.values[1].as_ref());
            assert_eq!(Value::String("bar".to_owned()), *option.values[2].as_ref());
            assert_eq!(Value::Number(12u32.into()), *option.values[3].as_ref());
            return;
        }

        panic!("option did not match");
    }
}

