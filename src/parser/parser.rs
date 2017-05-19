#![recursion_limit = "80"]

use std::collections::LinkedList;
use std::collections::BTreeMap;

use super::ast;
use super::errors::*;
use pest::prelude::*;

fn find_indent(input: &str) -> Option<usize> {
    let mut it = input.chars();

    let mut index = 0usize;

    while let Some(c) = it.next() {
        match c {
            ' ' | '\t' => {}
            _ => {
                return Some(index);
            }
        }

        index += 1;
    }

    None
}

fn strip_code_block(input: &str) -> Vec<String> {
    let mut indent: Option<usize> = None;

    let mut current_line = 0;
    let mut first_line = false;
    let mut empty_start = 0;
    let mut empty_end = 0;
    let mut it = input.lines();

    while let Some(line) = it.next() {
        current_line += 1;

        if let Some(current) = find_indent(line) {
            empty_end = current_line;

            if indent.map(|i| i > current).unwrap_or(true) {
                indent = Some(current);
            }

            first_line = true;
        } else {
            if !first_line {
                empty_start += 1;
            }
        }
    }

    if let Some(indent) = indent {
        let mut out: Vec<String> = Vec::new();
        let mut it = input.lines().skip(empty_start).take(empty_end - empty_start);

        while let Some(line) = it.next() {
            if line.len() < indent {
                out.push(line.to_owned());
            } else {
                let stripped = &line[indent..];
                out.push(stripped.to_owned());
            }
        }

        return out;
    }

    return input.lines().map(ToOwned::to_owned).collect();
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
        decl = { message_decl | interface_decl | type_decl }

        use_decl = { ["use"] ~ package_ident ~ use_as? ~ [";"] }
        use_as = { ["as"] ~ ident }
        package_decl = { ["package"] ~ package_ident ~ [";"] }
        message_decl = { ["message"] ~ ident ~ ["{"] ~ option_decl* ~ message_member* ~ ["}"] }
        interface_decl = { ["interface"] ~ ident ~ ["{"] ~ option_decl* ~ interface_member* ~ sub_type_decl* ~ ["}"] }
        type_decl = { ["type"] ~ ident ~ ["="] ~ type_spec ~ [";"] }
        sub_type_decl = { sub_type }
        sub_type = { ident ~ ["{"] ~ option_decl* ~ sub_type_member* ~ ["}"] }

        message_member = { field | code_block }
        interface_member = { field | code_block }
        sub_type_member = { field | code_block }

        field = { ident ~ modifier? ~ [":"] ~ type_spec ~ [";"] }
        code_block = @{ ident ~ whitespace* ~ ["@@"] ~ code_body ~ ["@@"] }
        code_body = { (!(["@@"]) ~ any)* }
        // body of a code block, either another balanced block, or anything but brackets
        modifier = { ["?"] }

        type_spec = { map | array | tuple | used_type | type_literal }
        type_literal = { ident }
        used_type = { ident ~ ["."] ~ ident }
        tuple = { ["("] ~ ( tuple_element ~ ([","] ~ tuple_element)* ) ~ [")"] }
        tuple_element = { (ident ~ [":"])? ~ type_spec }
        map = { ["{"] ~ type_spec ~ [":"] ~ type_spec ~ ["}"] }
        array = { ["["] ~ array_argument ~ ["]"] }
        array_argument = { type_spec }

        option_decl = { ident ~ (option_value ~ ([","] ~ option_value)*) ~ [";"] }

        option_value = { string | number }

        package_ident = @{ ident ~ (["."] ~ ident)* }
        ident =  @{ (['a'..'z'] | ['A'..'Z'] | ["_"]) ~ (['0'..'9'] | ['a'..'z'] | ['A'..'Z'] | ["_"])* }

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
            (token: message_decl, &name: ident, options: _option_list(), members: _message_member_list()) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let pos = (token.start, token.end);
                let message_decl = ast::MessageDecl::new(name.to_owned(), options, members, pos);
                ast::Decl::Message(message_decl)
            },

            (
                token: interface_decl,
                &name: ident,
                options: _option_list(),
                members: _interface_member_list(),
                sub_types: _sub_type_list()
            ) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let pos = (token.start, token.end);
                let interface_decl = ast::InterfaceDecl::new(name.to_owned(), options, members, sub_types, pos);
                ast::Decl::Interface(interface_decl)
            },

            (token: type_decl, &name: ident, type_spec: _type_spec()) => {
                let pos = (token.start, token.end);
                let type_decl = ast::TypeDecl::new(name.to_owned(), type_spec, pos);
                ast::Decl::Type(type_decl)
            },
        }

        _message_member_list(&self) -> LinkedList<ast::MessageMember> {
            (_: message_member, first: _message_member(), mut tail: _message_member_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _message_member(&self) -> ast::MessageMember {
            (token: field, field: _field()) => {
                let pos = (token.start, token.end);
                ast::MessageMember::Field(field, pos)
            },

            (token: code_block, &context: ident, &content: code_body) => {
                let pos = (token.start, token.end);
                ast::MessageMember::Code(context.to_owned(), strip_code_block(content), pos)
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
                ast::OptionValue::String(decode_escaped_string(string).unwrap())
            },
        }

        _sub_type_member_list(&self) -> LinkedList<ast::SubTypeMember> {
            (_: sub_type_member, first: _sub_type_member(), mut tail: _sub_type_member_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _sub_type_member(&self) -> ast::SubTypeMember {
            (_: field, field: _field()) => {
                ast::SubTypeMember::Field(field)
            },

            (token: code_block, &context: ident, &content: code_body) => {
                let pos = (token.start, token.end);
                ast::SubTypeMember::Code(context.to_owned(), strip_code_block(content), pos)
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
            (token: sub_type, &name: ident, options: _option_list(), members: _sub_type_member_list()) => {
                let name = name.to_owned();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let pos = (token.start, token.end);
                ast::SubType::new(name, options, members, pos)
            },
        }

        _interface_member_list(&self) -> LinkedList<ast::InterfaceMember> {
            (_: interface_member, first: _interface_member(), mut tail: _interface_member_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _interface_member(&self) -> ast::InterfaceMember {
            (token: field, field: _field()) => {
                let pos = (token.start, token.end);
                ast::InterfaceMember::Field(field, pos)
            },

            (token: code_block, &context: ident, &content: code_body) => {
                let pos = (token.start, token.end);
                ast::InterfaceMember::Code(context.to_owned(), strip_code_block(content), pos)
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

            (_: type_spec, _: tuple, arguments: _tuple_element_list()) => {
                let arguments = arguments.into_iter().collect();
                ast::Type::Tuple(arguments)
            },

            (_: type_spec, _: array, _: array_argument, argument: _type_spec()) => {
                ast::Type::Array(Box::new(argument))
            },

            (_: type_spec, _: map, key: _type_spec(), value: _type_spec()) => {
                ast::Type::Map(Box::new(key), Box::new(value))
            },
        }

        _tuple_element_list(&self) -> LinkedList<ast::TupleElement> {
            (_: tuple_element, first: _tuple_element(), mut tail: _tuple_element_list()) => {
                tail.push_front(first);
                tail
            },

            () => LinkedList::new(),
        }

        _tuple_element(&self) -> ast::TupleElement {
            (&name: ident, ty: _type_spec()) => {
                let name = Some(name.to_owned());
                ast::TupleElement::new(name, ty)
            },
            (ty: _type_spec()) => {
                ast::TupleElement::new(None, ty)
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
        let mut parser = Rdp::new(StringInput::new("[(string, string)]"));

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
        let mut parser = Rdp::new(StringInput::new("java@@\na { b { c } d } e\n@@"));

        assert!(parser.code_block());
        assert!(parser.end());
    }

    #[test]
    fn test_strip_code_block() {
        let result = strip_code_block("\n   hello\n  world\n\n\n again\n\n\n");
        assert_eq!(vec!["  hello", " world", "", "", "again"], result);
    }

    #[test]
    fn test_tuple() {
        let mut elements = Vec::new();
        elements.push(ast::TupleElement::new(Some("foo".to_owned()), ast::Type::String));
        elements.push(ast::TupleElement::new(Some("bar".to_owned()), ast::Type::String));
        elements.push(ast::TupleElement::new(None, ast::Type::U32));

        let reference = ast::Type::Tuple(elements);

        let mut parser = Rdp::new(StringInput::new("(foo: string, bar: string, u32)"));

        assert!(parser.type_spec());
        assert!(parser.end());

        let result = parser._type_spec();

        assert_eq!(reference, result);
    }
}
