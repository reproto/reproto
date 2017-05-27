use ast;
use backend::models as m;
use pest::prelude::*;
use std::collections::BTreeMap;
use std::collections::LinkedList;
use super::errors::*;

/// Check if character is an indentation character.
fn is_indent(c: char) -> bool {
    match c {
        ' ' | '\t' => true,
        _ => false,
    }
}

/// Find the number of whitespace characters that the given string is indented.
fn find_indent(input: &str) -> Option<usize> {
    input.chars().enumerate().find(|&(_, c)| !is_indent(c)).map(|(i, _)| i)
}

fn code_block_indent(input: &str) -> Option<(usize, usize, usize)> {
    let mut indent: Option<usize> = None;

    let mut start = 0;
    let mut end = 0;

    let mut first_line = false;

    for (line_no, line) in input.lines().enumerate() {
        if let Some(current) = find_indent(line) {
            end = line_no + 1;

            if indent.map(|i| i > current).unwrap_or(true) {
                indent = Some(current);
            }

            first_line = true;
        } else {
            if !first_line {
                start += 1;
            }
        }
    }

    indent.map(|indent| (indent, start, end - start))
}

/// Strip common indent from all input lines.
fn strip_code_block(input: &str) -> Vec<String> {
    if let Some((indent, empty_start, len)) = code_block_indent(input) {
        input.lines()
            .skip(empty_start)
            .take(len)
            .map(|line| {
                if line.len() < indent {
                    line.to_owned()
                } else {
                    (&line[indent..]).to_owned()
                }
            })
            .collect()
    } else {
        input.lines().map(ToOwned::to_owned).collect()
    }
}

/// Decode an escaped string.
fn decode_escaped_string(input: &str) -> Result<String> {
    let mut out = String::new();
    let mut it = input.chars().skip(1).peekable();

    loop {
        let c = match it.next() {
            None => break,
            Some(c) => c,
        };

        // strip end quote
        if it.peek().is_none() {
            break;
        }

        if c == '\\' {
            let escaped = match it.next().ok_or("expected character")? {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'u' => decode_unicode4(&mut it)?,
                _ => return Err(ErrorKind::InvalidEscape.into()),
            };

            out.push(escaped);
            continue;
        }

        out.push(c);
    }

    Ok(out)
}

/// Decode the next four characters as a unicode escape sequence.
fn decode_unicode4(it: &mut Iterator<Item = char>) -> Result<char> {
    let mut res = 0u32;

    for x in 0..4u32 {
        let c = it.next().ok_or("expected hex character")?.to_string();
        let c = u32::from_str_radix(&c, 16)?;
        res += c << (4 * (3 - x));
    }

    Ok(::std::char::from_u32(res).ok_or("expected valid character")?)
}

impl_rdp! {
    grammar! {
        file = _{ package_decl ~ use_decl* ~ decl* ~ eoi }
        decl = { type_decl | interface_decl | tuple_decl | enum_decl }

        use_decl = { ["use"] ~ package_ident ~ use_as? ~ [";"] }
        use_as = { ["as"] ~ ident }
        package_decl = { ["package"] ~ package_ident ~ [";"] }
        type_decl = { ["type"] ~ ident ~ ["{"] ~ option_decl* ~ member* ~ ["}"] }
        tuple_decl = { ["tuple"] ~ ident ~ ["{"] ~ option_decl* ~ member* ~ ["}"] }
        interface_decl = { ["interface"] ~ ident ~ ["{"] ~ option_decl* ~ member* ~ sub_type* ~ ["}"] }
        enum_decl = {
            ["enum"] ~ ident ~ ["{"] ~ (enum_value ~ [","])* ~ enum_value ~ [";"] ~ option_decl* ~ member* ~ ["}"]
        }
        sub_type = { ident ~ ["{"] ~ option_decl* ~ member* ~ ["}"] }

        member = { field | code_block }

        field = { ident ~ modifier? ~ [":"] ~ type_spec ~ [";"] }
        code_block = @{ ident ~ whitespace* ~ ["{{"] ~ code_body ~ ["}}"] }
        code_body = { (!(["}}"]) ~ any)* }
        // body of a code block, either another balanced block, or anything but brackets
        modifier = { ["?"] }

        enum_value = { ident ~ (["("] ~ (value ~ ([","] ~ value)*) ~ [")"])? }
        option_decl = { ident ~ (option_value ~ ([","] ~ option_value)*) ~ [";"] }

        option_value = { string | integer | ident }

        package_ident = @{ ident ~ (["."] ~ ident)* }

        type_spec = _{
            float_type |
            double_type |
            signed_type |
            unsigned_type |
            boolean_type |
            string_type |
            bytes_type |
            any_type |
            map_type |
            array_type |
            used_type |
            custom_type
        }

        float_type = @{ ["float"] }
        double_type = @{ ["double"] }
        signed_type = @{ ["signed"] ~ type_bits? }
        unsigned_type = @{ ["unsigned"] ~ type_bits? }
        boolean_type = @{ ["boolean"] }
        string_type = @{ ["string"] }
        bytes_type = @{ ["bytes"] }
        any_type = @{ ["any"] }
        map_type = { ["{"] ~ type_spec ~ [":"] ~ type_spec ~ ["}"] }
        array_type = { ["["] ~ type_spec ~ ["]"] }
        used_type = @{ ident ~ ["."] ~ ident }
        custom_type = { ident }

        type_bits = { (["/"] ~ integer) }

        value = { string | float | integer | boolean }

        ident =  @{ (['a'..'z'] | ['A'..'Z'] | ["_"]) ~ (['0'..'9'] | ['a'..'z'] | ['A'..'Z'] | ["_"])* }

        string  = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        escape  =  _{ ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["n"] | ["r"] | ["t"] | unicode) }
        unicode =  _{ ["u"] ~ hex ~ hex ~ hex ~ hex }
        hex     =  _{ ['0'..'9'] | ['a'..'f'] }

        integer = @{ ["-"]? ~ int }
        float  = @{ ["-"]? ~ int? ~ (["."] ~ ['0'..'9']+) }
        int    =  _{ ["0"] | ['1'..'9'] ~ ['0'..'9']* }

        boolean = { ["true"] | ["false"] }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }

        comment = _{
            // line comment
            ( ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) ) |
            // block comment
            ( ["/*"] ~ (!(["*/"]) ~ any)* ~ ["*/"] )
        }
    }

    process! {
        _file(&self) -> ast::File {
            (_: package_decl, package: _package(), uses: _use_list(), decls: _decl_list()) => {
                let uses = uses.into_iter().collect();
                let decls = decls.into_iter().collect();

                ast::File {
                    package: package,
                    uses: uses,
                    decls: decls
                }
            },
        }

        _use_list(&self) -> LinkedList<ast::Token<ast::UseDecl>> {
            (token: use_decl, package: _package(), alias: _use_as(), mut tail: _use_list()) => {
                let use_decl = ast::UseDecl {
                    package: package,
                    alias: alias,
                };

                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(use_decl, pos));

                tail
            },

            () => LinkedList::new(),
        }

        _use_as(&self) -> Option<String> {
            (_: use_as, &alias: ident) => {
                Some(alias.to_owned())
            },

            () => None,
        }

        _package(&self) -> ast::Token<m::Package> {
            (token: package_ident, idents: _ident_list()) => {
                let package = m::Package::new(idents.into_iter().collect());
                let pos = (token.start, token.end);
                ast::Token::new(package, pos)
            },
        }

        _decl_list(&self) -> LinkedList<ast::Token<ast::Decl>> {
            (token: decl, first: _decl(), mut tail: _decl_list()) => {
                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(first, pos));
                tail
            },

            () => LinkedList::new(),
        }

        _decl(&self) -> ast::Decl {
            (
                _: type_decl,
                &name: ident,
                options: _option_list(),
                members: _member_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let body = ast::TypeBody {
                    name: name.to_owned(),
                    options: options,
                    members: members
                };

                ast::Decl::Type(body)
            },

            (
                _: tuple_decl,
                &name: ident,
                options: _option_list(),
                members: _member_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let body = ast::TupleBody {
                    name: name.to_owned(),
                    options: options,
                    members: members,
                };

                ast::Decl::Tuple(body)
            },

            (
                _: interface_decl,
                &name: ident,
                options: _option_list(),
                members: _member_list(),
                sub_types: _sub_type_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let body = ast::InterfaceBody {
                    name: name.to_owned(),
                    options: options,
                    members: members,
                    sub_types: sub_types,
                };

                ast::Decl::Interface(body)
            },

            (
                _: enum_decl,
                &name: ident,
                values: _enum_value_list(),
                options: _option_list(),
                members: _member_list(),
            ) => {
                let values = values.into_iter().collect();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let body = ast::EnumBody {
                    name: name.to_owned(),
                    values: values,
                    options: options,
                    members: members,
                };

                ast::Decl::Enum(body)
            },
        }

        _enum_value_list(&self) -> LinkedList<ast::Token<ast::EnumValue>> {
            (token: enum_value, first: _enum_value(), mut tail: _enum_value_list()) => {
                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(first, pos));
                tail
            },

            () => LinkedList::new(),
        }

        _enum_value(&self) -> ast::EnumValue {
            (&name: ident, values: _value_list()) => {
                let arguments = values.into_iter().collect();

                ast::EnumValue {
                    name: name.to_owned(),
                    arguments: arguments,
                }
            },
        }

        _value_list(&self) -> LinkedList<ast::Token<m::Value>> {
            (token: value, first: _value(), mut tail: _value_list()) => {
                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(first, pos));
                tail
            },

            () => LinkedList::new(),
        }

        _value(&self) -> m::Value {
            (&value: string) => {
                let value = decode_escaped_string(value).unwrap();
                m::Value::String(value)
            },

            (value: _signed()) => {
                m::Value::Integer(value)
            },

            (&value: float) => {
                let value = value.parse::<f64>().unwrap();
                m::Value::Float(value)
            },

            (&value: boolean) => {
                let value = match value {
                    "true" => true,
                    "false" => false,
                    _ => panic!("should not happen"),
                };

                m::Value::Boolean(value)
            },
        }

        _signed(&self) -> i64 {
            (&value: integer) => {
                value.parse::<i64>().unwrap()
            },
        }

        _usize(&self) -> usize {
            (&value: integer) => {
                value.parse::<usize>().unwrap()
            },
        }

        _unsigned(&self) -> u64 {
            (&value: integer) => {
                value.parse::<u64>().unwrap()
            },
        }

        _option_list(&self) -> LinkedList<ast::Token<ast::OptionDecl>> {
            (token: option_decl, first: _option_decl(), mut tail: _option_list()) => {
                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(first, pos));
                tail
            },

            () => LinkedList::new(),
        }

        _option_decl(&self) -> ast::OptionDecl {
            (&name: ident, values: _option_value_list()) => {
                let values = values.into_iter().collect();

                ast::OptionDecl {
                    name: name.to_owned(),
                    values: values,
                }
            },
        }

        _option_value_list(&self) -> LinkedList<ast::Token<ast::OptionValue>> {
            (token: option_value, first: _option_value(), mut tail: _option_value_list()) => {
                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(first, pos));
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _option_value(&self) -> ast::OptionValue {
            (&string: string) => {
                let string = decode_escaped_string(string).unwrap();
                ast::OptionValue::String(string)
            },

            (&value: ident) => {
                ast::OptionValue::Identifier(value.to_owned())
            },

            (&value: integer) => {
                let value = value.parse::<i64>().unwrap();
                ast::OptionValue::Integer(value)
            },
        }

        _member_list(&self) -> LinkedList<ast::Token<ast::Member>> {
            (token: member, first: _member(), mut tail: _member_list()) => {
                let pos = (token.start, token.end);
                tail.push_front(ast::Token::new(first, pos));
                tail
            },

            () => LinkedList::new(),
        }

        _member(&self) -> ast::Member {
            (_: field, field: _field()) => {
                ast::Member::Field(field)
            },

            (_: code_block, &context: ident, &content: code_body) => {
                let block = strip_code_block(content);
                ast::Member::Code(context.to_owned(), block)
            },
        }

        _sub_type_list(&self) -> BTreeMap<String, ast::Token<ast::TypeBody>> {
            (token: sub_type, first: _sub_type(), mut tail: _sub_type_list()) => {
                let pos = (token.start, token.end);
                tail.insert(first.name.clone(), ast::Token::new(first, pos));
                tail
            },

            () => {
                BTreeMap::new()
            },
        }

        _sub_type(&self) -> ast::TypeBody {
            (&name: ident, options: _option_list(), members: _member_list()) => {
                let name = name.to_owned();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();

                ast::TypeBody {
                    name: name,
                    options: options,
                    members: members,
                }
            },
        }

        _field(&self) -> ast::Field {
            (&name: ident, modifier: _modifier(), type_spec: _type_spec()) => {
                ast::Field::new(modifier, name.to_owned(), type_spec, 0)
            },
        }

        _type_spec(&self) -> m::Type {
            (_: double_type) => {
                m::Type::Double
            },

            (_: float_type) => {
                m::Type::Float
            },

            (_: signed_type, _: type_bits, size: _usize()) => {
                m::Type::Signed(Some(size))
            },

            (_: unsigned_type, _: type_bits, size: _usize()) => {
                m::Type::Unsigned(Some(size))
            },

            (_: signed_type) => {
                m::Type::Signed(None)
            },

            (_: unsigned_type) => {
                m::Type::Unsigned(None)
            },

            (_: boolean_type) => {
                m::Type::Boolean
            },

            (_: string_type) => {
                m::Type::String
            },

            (_: bytes_type) => {
                m::Type::Bytes
            },

            (_: any_type) => {
                m::Type::Any
            },

            (_: custom_type, &name: ident) => {
                m::Type::Custom(name.to_owned())
            },

            (_: used_type, &used: ident, &value: ident) => {
                m::Type::UsedType(used.to_owned(), value.to_owned())
            },

            (_: array_type, argument: _type_spec()) => {
                m::Type::Array(Box::new(argument))
            },

            (_: map_type, key: _type_spec(), value: _type_spec()) => {
                m::Type::Map(Box::new(key), Box::new(value))
            },
        }

        _modifier(&self) -> m::Modifier {
            (_: modifier) => m::Modifier::Optional,
            () => m::Modifier::Required,
        }

        _ident_list(&self) -> LinkedList<String> {
            (&first: ident, mut tail: _ident_list()) => {
                tail.push_front(first.to_owned());
                tail
            },

            () => LinkedList::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE1: &[u8] = include_bytes!("tests/file1.reproto");

    #[test]
    fn test_file1() {
        let input = ::std::str::from_utf8(FILE1).unwrap();
        let mut parser = Rdp::new(StringInput::new(input));

        assert!(parser.file());
        assert!(parser.end());

        let file = parser._file();

        let package = m::Package::new(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]);

        assert_eq!(package, *file.package);
        assert_eq!(4, file.decls.len());
    }

    #[test]
    fn test_array() {
        let mut parser = Rdp::new(StringInput::new("[string]"));

        assert!(parser.type_spec());
        assert!(parser.end());

        parser._type_spec();
    }

    #[test]
    fn test_block_comment() {
        let mut parser = Rdp::new(StringInput::new("/* hello \n world */"));

        assert!(parser.comment());
    }

    #[test]
    fn test_line_comment() {
        let mut parser = Rdp::new(StringInput::new("// hello world\n"));

        assert!(parser.comment());
    }

    #[test]
    fn test_code_block() {
        let mut parser = Rdp::new(StringInput::new("a { b { c } d } e"));

        assert!(parser.code_body());
        assert!(parser.end());
    }

    #[test]
    fn test_code() {
        let mut parser = Rdp::new(StringInput::new("java{{\na { b { c } d } e\n}}"));

        assert!(parser.code_block());
        assert!(parser.end());
    }

    #[test]
    fn test_find_indent() {
        assert_eq!(Some(4), find_indent("   \thello"));
        assert_eq!(Some(0), find_indent("nope"));
        assert_eq!(None, find_indent(""));
        assert_eq!(None, find_indent("    "));
    }

    #[test]
    fn test_strip_code_block() {
        let result = strip_code_block("\n   hello\n  world\n\n\n again\n\n\n");
        assert_eq!(vec!["  hello", " world", "", "", "again"], result);
    }

    #[test]
    fn test_value() {
        let mut parser = Rdp::new(StringInput::new("60000.0"));

        assert!(parser.value());
        assert!(parser.end());
    }

    #[test]
    fn test_interface() {
        let input = "package foo.bar; interface Foo { reserved 1, 2, 3; java {{  }} Hello { } \
                     World { } }";

        let mut parser = Rdp::new(StringInput::new(input));

        assert!(parser.file());
        assert!(parser.end());

        let file = parser._file();

        assert_eq!(1, file.decls.len());
    }
}
