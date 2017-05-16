#![recursion_limit = "80"]

use std::collections::LinkedList;
use std::collections::BTreeMap;

use super::ast;
use super::errors::*;
use pest::prelude::*;

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
        file = _{ package_decl ~ imports ~ decl* ~ eoi }
        decl = { message_decl | interface_decl | type_decl }

        imports = { import_decl* }
        import_decl = { ["use"] ~ package_identifier ~ [";"] }
        package_decl = { ["package"] ~ package_identifier ~ [";"] }
        message_decl = { ["message"] ~ identifier ~ ["{"] ~ option* ~ message_member* ~ ["}"] }
        interface_decl = { ["interface"] ~ identifier ~ ["{"] ~ option* ~ interface_member* ~ sub_type* ~ ["}"] }
        type_decl = { ["type"] ~ identifier ~ ["="] ~ type_spec ~ [";"] }
        sub_type = { identifier ~ ["{"] ~ option* ~ sub_type_member* ~ ["}"] }

        message_member = { field }
        interface_member = { field }
        sub_type_member = { field }

        field = { identifier ~ modifier? ~ [":"] ~ type_spec ~ [";"] }

        type_spec = { array | tuple | used_type | type_literal }
        type_literal = { identifier }
        used_type = { identifier ~ ["."] ~ identifier }
        tuple = { ["("] ~ ( tuple_element ~ ([","] ~ tuple_element)* ) ~ [")"] }
        tuple_element = { type_spec }
        array = { ["["] ~ array_argument ~ ["]"] }
        array_argument = { type_spec }

        modifier = { ["?"] }

        option = { identifier ~ (option_value ~ ([","] ~ option_value)*) ~ [";"] }

        option_value = { string | number }

        package_identifier = @{ identifier ~ (["."] ~ identifier)* }
        identifier =  @{ (['a'..'z'] | ['A'..'Z']) ~ (['0'..'9'] | ['a'..'z'] | ['A'..'Z'])* }

        string  = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        escape  =  { ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["n"] | ["r"] | ["t"] | unicode) }
        unicode =  { ["u"] ~ hex ~ hex ~ hex ~ hex }
        hex     =  { ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }

        number = @{ ["-"]? ~ int ~ (["."] ~ ['0'..'9']+ ~ exp? | exp)? }
        int    =  { ["0"] | ['1'..'9'] ~ ['0'..'9']* }
        exp    =  { (["E"] | ["e"]) ~ (["+"] | ["-"])? ~ int }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }

        comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
    }

    process! {
        process_file(&self) -> Result<ast::File> {
            (package: _package(), imports: _imports(), decls: _decls()) => {
                let imports = imports.into_iter().collect();
                let decls = decls.into_iter().collect();
                Ok(ast::File::new(package, imports, decls))
            },
        }

        _package(&self) -> ast::Package {
            (_: package_decl, _: package_identifier, identifiers: _identifiers()) => {
                ast::Package::new(identifiers.into_iter().collect())
            },
        }

        _imports(&self) -> LinkedList<ast::Package> {
            (_: imports, first: _import(), mut tail: _imports()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _import(&self) -> ast::Package {
            (_: import_decl, _: package_identifier, identifiers: _identifiers()) => {
                ast::Package::new(identifiers.into_iter().collect())
            },
        }

        _decls(&self) -> LinkedList<ast::Decl> {
            (_: decl, first: _decl(), mut tail: _decls()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _decl(&self) -> ast::Decl {
            (_: message_decl, &name: identifier, options: _options(), members: _message_members()) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let m = ast::MessageDecl::new(name.to_owned(), options, members);
                ast::Decl::Message(m)
            },

            (_: interface_decl, &name: identifier, options: _options(), members: _interface_members(), sub_types: _sub_types()) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let m = ast::InterfaceDecl::new(name.to_owned(), options, members, sub_types);
                ast::Decl::Interface(m)
            },

            (_: type_decl, &name: identifier, type_: _type_spec()) => {
                let t = ast::TypeDecl::new(name.to_owned(), type_);
                ast::Decl::Type(t)
            },
        }

        _message_members(&self) -> LinkedList<ast::MessageMember> {
            (_: message_member, first: _message_member(), mut tail: _message_members()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _message_member(&self) -> ast::MessageMember {
            (_: field, field: _field()) => {
                ast::MessageMember::Field(field)
            },
        }

        _options(&self) -> LinkedList<ast::OptionPair> {
            (_: option, first: _option(), mut tail: _options()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _option(&self) -> ast::OptionPair {
            (&name: identifier, values: _option_values()) => {
                let values = values.into_iter().collect();
                ast::OptionPair::new(name.to_owned(), values)
            },
        }

        _option_values(&self) -> LinkedList<ast::OptionValue> {
            (_: option_value, first: _option_value(), mut tail: _option_values()) => {
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

        _sub_type_members(&self) -> LinkedList<ast::SubTypeMember> {
            (_: sub_type_member, first: _sub_type_member(), mut tail: _sub_type_members()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _sub_type_member(&self) -> ast::SubTypeMember {
            (_: field, field: _field()) => {
                ast::SubTypeMember::Field(field)
            },
        }

        _sub_types(&self) -> BTreeMap<String, ast::SubType> {
            (_: sub_type, first: _sub_type(), mut tail: _sub_types()) => {
                tail.insert(first.name.clone(), first);
                tail
            },

            () => {
                BTreeMap::new()
            },
        }

        _sub_type(&self) -> ast::SubType {
            (&name: identifier, options: _options(), members: _sub_type_members()) => {
                let name = name.to_owned();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                ast::SubType::new(name, options, members)
            },
        }

        _interface_members(&self) -> LinkedList<ast::InterfaceMember> {
            (_: interface_member, first: _interface_member(), mut tail: _interface_members()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _interface_member(&self) -> ast::InterfaceMember {
            (_: field, field: _field()) => {
                ast::InterfaceMember::Field(field)
            },
        }

        _field(&self) -> ast::Field {
            (&name: identifier, modifier: _modifier(), type_: _type_spec()) => {
                ast::Field::new(modifier, name.to_owned(), type_, 0)
            },
        }

        _type_spec(&self) -> ast::Type {
            (_: type_spec, _: type_literal, &value: identifier) => {
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

            (_: type_spec, _: used_type, &used: identifier, &value: identifier) => {
                ast::Type::UsedType(used.to_owned(), value.to_owned())
            },

            (_: type_spec, _: tuple, arguments: _tuple_elements()) => {
                let arguments = arguments.into_iter().collect();
                ast::Type::Tuple(arguments)
            },

            (_: type_spec, _: array, _: array_argument, argument: _type_spec()) => {
                ast::Type::Array(Box::new(argument))
            },
        }

        _tuple_elements(&self) -> LinkedList<ast::Type> {
            (_: tuple_element, first: _type_spec(), mut tail: _tuple_elements()) => {
                tail.push_front(first);
                tail
            },

            () => {
                LinkedList::new()
            },
        }

        _modifier(&self) -> ast::Modifier {
            (_: modifier) => {
                ast::Modifier::Optional
            },

            () => {
                ast::Modifier::Required
            },
        }

        _identifiers(&self) -> LinkedList<String> {
            (&first: identifier, mut tail: _identifiers()) => {
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

    fn load_interface() -> ast::Decl {
        let input = ::std::str::from_utf8(include_bytes!("tests/interface")).unwrap();
        let mut parser = Rdp::new(StringInput::new(input));

        assert!(parser.decl());

        parser._decls().into_iter().next().unwrap()
    }

    #[test]
    fn test_file1() {
        let input = ::std::str::from_utf8(include_bytes!("tests/file1.reproto")).unwrap();
        let mut parser = Rdp::new(StringInput::new(input));
        assert!(parser.file());
        assert!(parser.end());
        let file = parser.process_file().unwrap();

        let package = ast::Package::new(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]);

        assert_eq!(package, file.package);
        assert_eq!(4, file.decls.len());

        assert_eq!(load_interface(), file.decls[2]);
    }

    #[test]
    fn test_array() {
        let mut parser = Rdp::new(StringInput::new("[(string, string)]"));
        assert!(parser.type_spec());
        assert!(parser.end());
        parser._type_spec();
    }
}
