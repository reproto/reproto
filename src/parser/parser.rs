#![recursion_limit = "80"]

use std::collections::{LinkedList, HashSet};

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
        file = _{ package_decl ~ decl* ~ eoi }
        decl = { message_decl | interface_decl }

        package_decl = { ["package"] ~ package_identifier ~ [";"] }
        message_decl = { ["message"] ~ identifier ~ ["{"] ~ option* ~ message_member* ~ ["}"] }
        interface_decl = { ["interface"] ~ identifier ~ ["{"] ~ option* ~ interface_member* ~ ["}"] }
        sub_type = { identifier ~ ["{"] ~ option* ~ sub_type_member* ~ ["}"] }

        message_member = { field }
        interface_member = { sub_type | field }
        sub_type_member = { field }

        field = { modifier* ~ identifier ~ identifier ~ [";"] }

        modifier = { ["optional"] | ["required"] }

        option = { identifier ~ (option_value ~ ([","] ~ option_value)*) ~ [";"] }

        option_value = { string | number }

        package_identifier = @{ identifier ~ (["."] ~ identifier)* }
        identifier =  @{ (['a'..'z'] | ['A'..'z']) ~ (['0'..'9'] | ['a'..'z'] | ['A'..'z'])* }

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
            (package: _package(), decls: _decls()) => {
                let decls = decls.into_iter().collect();
                Ok(ast::File::new(package, decls))
            },
        }

        _package(&self) -> ast::Package {
            (_: package_decl, _: package_identifier, identifiers: _identifiers()) => {
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

            (_: interface_decl, &name: identifier, options: _options(), members: _interface_members()) => {
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let m = ast::InterfaceDecl::new(name.to_owned(), options, members);
                ast::Decl::Interface(m)
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
            (_: sub_type, &name: identifier, options: _options(), members: _sub_type_members()) => {
                let name = name.to_owned();
                let options = ast::Options::new(options.into_iter().collect());
                let members = members.into_iter().collect();
                let s = ast::SubType::new(name, options, members);
                ast::InterfaceMember::SubType(s)
            },

            (_: field, field: _field()) => {
                ast::InterfaceMember::Field(field)
            },
        }

        _field(&self) -> ast::Field {
            (modifiers: _modifiers(), type_: _type(), &name: identifier) => {
                let modifiers = ast::Modifiers::new(modifiers);
                ast::Field::new(modifiers, name.to_owned(), type_, 0)
            }
        }

        _type(&self) -> ast::Type {
            (&value: identifier) => {
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
                    name => ast::Type::Custom(name.to_owned()),
                }
            }
        }

        _modifiers(&self) -> HashSet<ast::Modifier> {
            (&first: modifier, mut tail: _modifiers()) => {
                tail.insert(match first {
                    "required" => ast::Modifier::Required,
                    "optional" => ast::Modifier::Optional,
                    _ => unreachable!(),
                });
                tail
            },

            () => HashSet::new(),
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
        assert_eq!(2, file.decls.len());

        assert_eq!(load_interface(), file.decls[1]);

        println!("file = {:?}", file);
    }
}
