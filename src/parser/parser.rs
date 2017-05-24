use pest::prelude::*;
use std::collections::BTreeMap;
use std::collections::LinkedList;

use super::ast;
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
        interface_decl = { ["interface"] ~ ident ~ ["{"] ~ option_decl* ~ member* ~ sub_type_decl* ~ ["}"] }
        enum_decl = {
            ["enum"] ~ ident ~ ["{"] ~ (enum_value ~ [","])* ~ enum_value ~ [";"] ~ option_decl* ~ member* ~ ["}"]
        }
        sub_type_decl = { sub_type }
        sub_type = { ident ~ ["{"] ~ option_decl* ~ member* ~ ["}"] }

        member = { field | code_block }

        field = { ident ~ modifier? ~ [":"] ~ type_spec ~ [";"] }
        code_block = @{ ident ~ whitespace* ~ ["{{"] ~ code_body ~ ["}}"] }
        code_body = { (!(["}}"]) ~ any)* }
        // body of a code block, either another balanced block, or anything but brackets
        modifier = { ["?"] }

        type_spec = { map | array | used_type | type_literal }
        type_literal = { ident }
        used_type = { ident ~ ["."] ~ ident }
        map = { ["{"] ~ type_spec ~ [":"] ~ type_spec ~ ["}"] }
        array = { ["["] ~ array_argument ~ ["]"] }
        array_argument = { type_spec }

        enum_value = { ident ~ (["("] ~ (literal ~ ([","] ~ literal)*) ~ [")"])? }
        option_decl = { ident ~ (option_value ~ ([","] ~ option_value)*) ~ [";"] }

        option_value = { string | number }

        package_ident = @{ ident ~ (["."] ~ ident)* }
        ident =  @{ (['a'..'z'] | ['A'..'Z'] | ["_"]) ~ (['0'..'9'] | ['a'..'z'] | ['A'..'Z'] | ["_"])* }

        literal = { string | number }

        string  = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        escape  =  _{ ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["n"] | ["r"] | ["t"] | unicode) }
        unicode =  _{ ["u"] ~ hex ~ hex ~ hex ~ hex }
        hex     =  _{ ['0'..'9'] | ['a'..'f'] }

        number = @{ ["-"]? ~ int ~ (["."] ~ ['0'..'9']+ ~ exp? | exp)? }
        int    =  _{ ["0"] | ['1'..'9'] ~ ['0'..'9']* }
        exp    =  _{ (["E"] | ["e"]) ~ (["+"] | ["-"])? ~ int }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }

        comment = _{
            // line comment
            ( ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) ) |
            // block comment
            ( ["/*"] ~ (!(["*/"]) ~ any)* ~ ["*/"] )
        }
    }

    process! {
        _file(&self) -> Result<ast::File> {
            (_: package_decl, package: _package(), imports: _use_list(), decls: _decl_list()) => {
                let imports = imports.into_iter().collect();
                let decls = decls.into_iter().collect();
                Ok(ast::File::new(package, imports, decls))
            },
        }

        _use_list(&self) -> LinkedList<ast::UseDecl> {
            (_: use_decl, package: _package(), alias: _use_as(), mut tail: _use_list()) => {
                tail.push_front(ast::UseDecl::new(package, alias));
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

        _package(&self) -> ast::Package {
            (_: package_ident, idents: _ident_list()) => {
                ast::Package::new(idents.into_iter().collect())
            },
        }

        _decl_list(&self) -> LinkedList<ast::Decl> {
            (_: decl, first: _decl(), mut tail: _decl_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _decl(&self) -> ast::Decl {
            (
                token: type_decl,
                &name: ident,
                options: _option_list(),
                members: _member_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let body = ast::TypeBody::new(name.to_owned(), options, members);
                let pos = (token.start, token.end);
                ast::Decl::Type(body, pos)
            },

            (
                token: tuple_decl,
                &name: ident,
                options: _option_list(),
                members: _member_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let body = ast::TupleBody::new(name.to_owned(), options, members);
                let pos = (token.start, token.end);
                ast::Decl::Tuple(body, pos)
            },

            (
                token: interface_decl,
                &name: ident,
                options: _option_list(),
                members: _member_list(),
                sub_types: _sub_type_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let pos = (token.start, token.end);
                let body = ast::InterfaceBody::new(name.to_owned(), options, members, sub_types);
                ast::Decl::Interface(body, pos)
            },

            (
                token: enum_decl,
                &name: ident,
                values: _enum_value_list(),
                options: _option_list(),
                members: _member_list(),
            ) => {
                let values = values.into_iter().collect();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let pos = (token.start, token.end);
                let body = ast::EnumBody::new(name.to_owned(), values, options, members);
                ast::Decl::Enum(body, pos)
            },
        }

        _enum_value_list(&self) -> LinkedList<ast::EnumValue> {
            (_: enum_value, first: _enum_value(), mut tail: _enum_value_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _enum_value(&self) -> ast::EnumValue {
            (&name: ident, values: _literal_list()) => {
                let values = values.into_iter().collect();
                ast::EnumValue::new(name.to_owned(), values)
            },
        }

        _literal_list(&self) -> LinkedList<ast::Literal> {
            (_: literal, first: _literal(), mut tail: _literal_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _literal(&self) -> ast::Literal {
            (&value: string) => {
                ast::Literal::String(value.to_owned())
            },

            (&value: number) => {
                let value = value.parse::<i64>().unwrap();
                ast::Literal::Number(value)
            },
        }

        _option_list(&self) -> LinkedList<ast::OptionDecl> {
            (_: option_decl, first: _option_decl(), mut tail: _option_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _option_decl(&self) -> ast::OptionDecl {
            (&name: ident, values: _option_value_list()) => {
                let values = values.into_iter().collect();
                ast::OptionDecl::new(name.to_owned(), values)
            },
        }

        _option_value_list(&self) -> LinkedList<ast::OptionValue> {
            (_: option_value, first: _option_value(), mut tail: _option_value_list()) => {
                tail.push_front(first);
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
        }

        _member_list(&self) -> LinkedList<ast::Member> {
            (_: member, first: _member(), mut tail: _member_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _member(&self) -> ast::Member {
            (token: field, field: _field()) => {
                let pos = (token.start, token.end);
                ast::Member::Field(field, pos)
            },

            (token: code_block, &context: ident, &content: code_body) => {
                let pos = (token.start, token.end);
                let block = strip_code_block(content);
                ast::Member::Code(context.to_owned(), block, pos)
            },
        }

        _sub_type_list(&self) -> BTreeMap<String, ast::SubType> {
            (_: sub_type_decl, first: _sub_type(), mut tail: _sub_type_list()) => {
                tail.insert(first.name.clone(), first);
                tail
            },

            () => {
                BTreeMap::new()
            },
        }

        _sub_type(&self) -> ast::SubType {
            (token: sub_type, &name: ident, options: _option_list(), members: _member_list()) => {
                let name = name.to_owned();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let pos = (token.start, token.end);
                ast::SubType::new(name, options, members, pos)
            },
        }

        _field(&self) -> ast::Field {
            (&name: ident, modifier: _modifier(), type_spec: _type_spec()) => {
                ast::Field::new(modifier, name.to_owned(), type_spec, 0)
            },
        }

        _type_spec(&self) -> ast::Type {
            (_: type_spec, _: type_literal, &value: ident) => {
                match value {
                    "double" => ast::Type::Double,
                    "float" => ast::Type::Float,
                    "i32" => ast::Type::I32,
                    "i64" => ast::Type::I64,
                    "u32" => ast::Type::U32,
                    "u64" => ast::Type::U64,
                    "bool" => ast::Type::Bool,
                    "string" => ast::Type::String,
                    "bytes" => ast::Type::Bytes,
                    "any" => ast::Type::Any,
                    name => ast::Type::Custom(name.to_owned()),
                }
            },

            (_: type_spec, _: used_type, &used: ident, &value: ident) => {
                ast::Type::UsedType(used.to_owned(), value.to_owned())
            },

            (_: type_spec, _: array, _: array_argument, argument: _type_spec()) => {
                ast::Type::Array(Box::new(argument))
            },

            (_: type_spec, _: map, key: _type_spec(), value: _type_spec()) => {
                ast::Type::Map(Box::new(key), Box::new(value))
            },
        }

        _modifier(&self) -> ast::Modifier {
            (_: modifier) => ast::Modifier::Optional,
            () => ast::Modifier::Required,
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

        let file = parser._file().unwrap();

        let package = ast::Package::new(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]);

        assert_eq!(package, file.package);
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
}
