#![allow(unconditional_recursion)]

use num_bigint::BigInt;
use pest::prelude::*;
use super::ast::*;
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

impl<T> From<(T, Token<Rule>)> for AstLoc<T> {
    fn from(pair: (T, Token<Rule>)) -> AstLoc<T> {
        AstLoc::new(pair.0, (pair.1.start, pair.1.end))
    }
}

impl_rdp! {
    grammar! {
        file = _{ package_decl ~ use_decl* ~ decl* ~ eoi }
        decl = { type_decl | interface_decl | tuple_decl | enum_decl }

        use_decl = { use_keyword ~ package_ident ~ use_as? ~ semi_colon }
        use_as = { as_keyword ~ identifier }

        package_decl = { package_keyword ~ package_ident ~ semi_colon }

        type_decl = { type_keyword ~ type_identifier ~ left_curly ~ type_body ~ right_curly }
        type_body = _{ member* }

        tuple_decl = { tuple_keyword ~ type_identifier ~ left_curly ~ tuple_body ~ right_curly }
        tuple_body = _{ member* }

        interface_decl = { interface_keyword ~ type_identifier ~ left_curly ~ interface_body ~ right_curly }
        interface_body = _{ member* ~ sub_type* }

        enum_decl = { enum_keyword ~ type_identifier ~ left_curly ~ enum_body ~ right_curly }
        enum_body = _{ enum_value* ~ member* }

        sub_type = { type_identifier ~ left_curly ~ sub_type_body ~ right_curly }
        sub_type_body = _{ member* }

        member = { option_decl | match_decl | field | code_block }
        field = { identifier ~ optional? ~ colon ~ type_spec ~ field_as? ~ semi_colon }
        field_as = { as_keyword ~ value }
        code_block = @{ identifier ~ whitespace* ~ code_start ~ code_body ~ code_end }
        code_body = { (!(["}}"]) ~ any)* }

        enum_value = { enum_name ~ enum_arguments? ~ enum_ordinal? ~ semi_colon }
        enum_name = { type_identifier }
        enum_arguments = { (left_paren ~ (value ~ (comma ~ value)*) ~ right_paren) }
        enum_ordinal = { equals ~ value }
        option_decl = { identifier ~ (value ~ (comma ~ value)*) ~ semi_colon }

        match_decl = { match_keyword ~ left_curly ~ match_member* ~ right_curly }
        match_member = { match_condition ~ hash_rocket ~ value ~ semi_colon }
        match_condition = { match_variable | match_value }
        match_variable = { identifier ~ colon ~ type_spec }
        match_value = { value }

        package_ident = @{ identifier ~ (dot ~ identifier)* }

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
            custom_type
        }

        // Types
        float_type = @{ ["float"] }
        double_type = @{ ["double"] }
        signed_type = @{ ["signed"] ~ type_bits? }
        unsigned_type = @{ ["unsigned"] ~ type_bits? }
        boolean_type = @{ ["boolean"] }
        string_type = @{ ["string"] }
        bytes_type = @{ ["bytes"] }
        any_type = @{ ["any"] }
        map_type = { left_curly ~ type_spec ~ colon ~ type_spec ~ right_curly }
        array_type = { bracket_start ~ type_spec ~ bracket_end }
        custom_type = @{ used_prefix? ~ type_identifier ~ (dot ~ type_identifier)* }

        used_prefix = @{ identifier ~ scope }

        // Keywords and tokens
        enum_keyword = @{ ["enum"] }
        use_keyword = @{ ["use"] }
        as_keyword = @{ ["as"] }
        package_keyword = @{ ["package"] }
        type_keyword = @{ ["type"] }
        tuple_keyword = @{ ["tuple"] }
        interface_keyword = @{ ["interface"] }
        match_keyword = @{ ["match"] }
        hash_rocket = @{ ["=>"] }
        comma = @{ [","] }
        colon = @{ [":"] }
        scope = @{ ["::"] }
        semi_colon = @{ [";"] }
        left_curly = @{ ["{"] }
        right_curly = @{ ["}"] }
        bracket_start = @{ ["["] }
        bracket_end = @{ ["]"] }
        code_start = @{ ["{{"] }
        code_end = @{ ["}}"] }
        left_paren = @{ ["("] }
        right_paren = @{ [")"] }
        forward_slash = @{ ["/"] }
        optional = @{ ["?"] }
        equals = @{ ["="] }
        dot = @{ ["."] }

        type_bits = _{ (forward_slash ~ unsigned) }

        optional_value_list = { value ~ (comma ~ value)* }
        value = { instance | constant | array | boolean | identifier | string | number }

        instance = { custom_type ~ instance_arguments }
        instance_arguments = { (left_paren ~ (field_init ~ (comma ~ field_init)*)? ~ right_paren) }

        constant = { custom_type }

        array = { bracket_start ~ optional_value_list? ~ bracket_end }

        field_init = { field_name ~ colon ~ value }
        field_name = { identifier }

        identifier = @{ ['a'..'z'] ~ (['0'..'9'] | ['a'..'z'] | ["_"])* }
        type_identifier = @{ ['A'..'Z'] ~ (['A'..'Z'] | ['a'..'z'] | ['0'..'9'])* }

        string  = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        escape  =  _{ ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["n"] | ["r"] | ["t"] | unicode) }
        unicode =  _{ ["u"] ~ hex ~ hex ~ hex ~ hex }
        hex     =  _{ ['0'..'9'] | ['a'..'f'] }

        unsigned = @{ int }
        number = @{ whole ~ (["."] ~  fraction)? ~ (["e"] ~ exponent)? }
        whole = { ["-"]? ~ int }
        fraction = { ['0'..'9']+ }
        exponent = { int }
        int      =  _{ ["0"] | ['1'..'9'] ~ ['0'..'9']* }

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
        process_file(&self) -> Result<File> {
            (
                _: package_decl,
                _: package_keyword,
                package: process_package(), _: semi_colon,
                uses: use_decl_list(),
                decls: decl_list(),
            ) => {
                let package = package;

                Ok(File {
                    package: package,
                    uses: uses,
                    decls: decls?
                })
            },
        }

        process_use_decl(&self) -> UseDecl {
            (_: use_keyword, package: process_package(), alias: process_use_as(), _: semi_colon) => {
                UseDecl {
                    package: package,
                    alias: alias,
                }
            }
        }

        process_use_as(&self) -> Option<String> {
            (_: use_as, _: as_keyword, &alias: identifier) => Some(alias.to_owned()),
            () => None,
        }

        process_package(&self) -> AstLoc<RpPackage> {
            (token: package_ident, idents: identifier_list()) => {
                (RpPackage::new(idents), token).into()
            },
        }

        process_decl(&self) -> Result<Decl> {
            (
                _: type_decl,
                _: type_keyword,
                &name: type_identifier,
                _: left_curly,
                members: member_list(),
                _: right_curly,
            ) => {
                let body = TypeBody {
                    name: name.to_owned(),
                    members: members?
                };

                Ok(Decl::Type(body))
            },

            (
                _: tuple_decl,
                _: tuple_keyword,
                &name: type_identifier,
                _: left_curly,
                members: member_list(),
                _: right_curly,
            ) => {
                let body = TupleBody {
                    name: name.to_owned(),
                    members: members?,
                };

                Ok(Decl::Tuple(body))
            },

            (
                _: interface_decl,
                _: interface_keyword,
                &name: type_identifier,
                _: left_curly,
                members: member_list(),
                sub_types: sub_type_list(),
                _: right_curly,
            ) => {
                let body = InterfaceBody {
                    name: name.to_owned(),
                    members: members?,
                    sub_types: sub_types?,
                };

                Ok(Decl::Interface(body))
            },

            (
                _: enum_decl,
                _: enum_keyword,
                &name: type_identifier,
                _: left_curly,
                values: enum_value_list(),
                members: member_list(),
                _: right_curly,
            ) => {
                let body = EnumBody {
                    name: name.to_owned(),
                    values: values?,
                    members: members?,
                };

                Ok(Decl::Enum(body))
            },
        }

        process_enum_value(&self) -> Result<EnumValue> {
            (
                name_token: enum_name,
                &name: type_identifier,
                values: process_enum_arguments(),
                ordinal: process_enum_ordinal(),
                _: semi_colon
             ) => {
                Ok(EnumValue {
                    name: (name.to_owned(), name_token).into(),
                    arguments: values?,
                    ordinal: ordinal?
                })
            },
        }

        process_enum_arguments(&self) -> Result<Vec<AstLoc<Value>>> {
            (_: enum_arguments, _: left_paren, values: value_list(), _: right_paren) => values,
            () => Ok(Vec::new()),
        }

        process_enum_ordinal(&self) -> Result<Option<AstLoc<Value>>> {
            (_: enum_ordinal, _: equals, value: process_value_token()) => value.map(Some),
            () => Ok(None),
        }

        process_optional_value_list(&self) -> Result<Vec<AstLoc<Value>>> {
            (_: optional_value_list, values: value_list()) => values,
            () => Ok(Vec::new()),
        }

        process_value_token(&self) -> Result<AstLoc<Value>> {
            (token: value, value: process_value()) => {
                value.map(move |v| (v, token).into())
            },
        }

        process_value(&self) -> Result<Value> {
            (
                token: instance,
                _: custom_type,
                name: process_name(),
                arguments_token: instance_arguments,
                _: left_paren,
                arguments: field_init_list(),
                _: right_paren,
            ) => {
                let instance = Instance {
                   ty: name,
                   arguments: (arguments?, arguments_token).into(),
                };

                Ok(Value::Instance((instance, token).into()))
            },

            (
                token: constant,
                _: custom_type,
                name: process_name(),
            ) => {
                Ok(Value::Constant((name, token).into()))
            },

            (
                _: array,
                _: bracket_start,
                values: process_optional_value_list(),
                _: bracket_end,
            ) => {
                Ok(Value::Array(values?))
            },

            (&value: string) => {
                let value = decode_escaped_string(value)?;
                Ok(Value::String(value))
            },

            (&value: identifier) => {
                Ok(Value::Identifier(value.to_owned()))
            },

            (
                _: number,
                &whole: whole,
                fraction: process_fraction(),
                exponent: process_exponent(),
            ) => {
                let whole = whole.parse::<BigInt>()?;
                let fraction = fraction?;
                let exponent = exponent?;

                Ok(Value::Number(RpNumber {
                    whole: whole, fraction: fraction, exponent: exponent
                }))
            },

            (&value: boolean) => {
                let value = match value {
                    "true" => true,
                    "false" => false,
                    _ => panic!("should not happen"),
                };

                Ok(Value::Boolean(value))
            },
        }

        process_fraction(&self) -> Result<Option<BigInt>> {
            (&fraction: fraction) => {
                Ok(Some(fraction.parse::<BigInt>()?))
            },

            () => Ok(None),
        }

        process_exponent(&self) -> Result<Option<i32>> {
            (&exponent: exponent) => {
                Ok(Some(exponent.parse::<i32>()?))
            },

            () => Ok(None),
        }

        process_used_prefix(&self) -> Option<String> {
            (_: used_prefix, &prefix: identifier, _: scope) => Some(prefix.to_owned()),
            () => None,
        }

        process_field_init(&self) -> Result<FieldInit> {
            (
                name_token: field_name,
                &name: identifier,
                _: colon,
                value: process_value_token(),
            ) => {
                Ok(FieldInit {
                    name: (name.to_owned(), name_token).into(),
                    value: value?,
                })
            },
        }

        process_member(&self) -> Result<Member> {
            (
                _: field,
                &name: identifier,
                modifier: process_modifier(),
                _: colon,
                type_spec: process_type_spec(),
                field_as: process_field_as(),
                _: semi_colon,
            ) => {
                let field = Field {
                    modifier: modifier,
                    name: name.to_owned(),
                    ty: type_spec?,
                    field_as: field_as?,
                };

                Ok(Member::Field(field))
            },

            (
                _: code_block,
                &context: identifier,
                _: code_start,
                &content: code_body,
                _: code_end,
             ) => {
                let block = strip_code_block(content);
                Ok(Member::Code(context.to_owned(), block))
            },

            (
                token: option_decl,
                &name: identifier,
                values: value_list(),
                _: semi_colon,
            ) => {
                let option_decl = OptionDecl { name: name.to_owned(), values: values? };
                Ok(Member::Option((option_decl, token).into()))
            },

            (
                _: match_decl,
                _: match_keyword,
                _: left_curly,
                members: match_member_list(),
                _: right_curly,
             ) => {
                let decl = MatchDecl {
                    members: members?,
                };

                Ok(Member::Match(decl))
            },
        }

        process_field_as(&self) -> Result<Option<AstLoc<Value>>> {
            (_: field_as, _: as_keyword, value: process_value_token()) => Ok(Some(value?)),
            () => Ok(None),
        }

        process_sub_type(&self) -> Result<SubType> {
            (
                &name: type_identifier,
                _: left_curly,
                members: member_list(),
                _: right_curly,
             ) => {
                let name = name.to_owned();
                Ok(SubType { name: name, members: members? })
            },
        }

        process_match_member(&self) -> Result<MatchMember> {
            (
                condition: process_match_condition(),
                _: hash_rocket,
                value: process_value_token(),
                _: semi_colon,
            ) => {
                Ok(MatchMember { condition: condition?, value: value? })
            },
        }

        process_match_condition(&self) -> Result<AstLoc<MatchCondition>> {
            (
                token: match_condition,
                _: match_value,
                value: process_value_token(),
            ) => {
                Ok((MatchCondition::Value(value?), token).into())
            },

            (
                token: match_condition,
                match_token: match_variable,
                &name: identifier,
                _: colon,
                ty: process_type_spec(),
            ) => {
                let variable = MatchVariable {
                    name: name.to_owned(),
                    ty: ty?,
                };

                Ok((MatchCondition::Type((variable, match_token).into()), token).into())
            },
        }

        process_type_spec(&self) -> Result<RpType> {
            (_: double_type) => {
                Ok(RpType::Double)
            },

            (_: float_type) => {
                Ok(RpType::Float)
            },

            (_: signed_type, _: forward_slash, &size: unsigned) => {
                let size = size.parse::<usize>()?;
                Ok(RpType::Signed(Some(size)))
            },

            (_: unsigned_type, _: forward_slash, &size: unsigned) => {
                let size = size.parse::<usize>()?;
                Ok(RpType::Unsigned(Some(size)))
            },

            (_: signed_type) => {
                Ok(RpType::Signed(None))
            },

            (_: unsigned_type) => {
                Ok(RpType::Unsigned(None))
            },

            (_: boolean_type) => {
                Ok(RpType::Boolean)
            },

            (_: string_type) => {
                Ok(RpType::String)
            },

            (_: bytes_type) => {
                Ok(RpType::Bytes)
            },

            (_: any_type) => {
                Ok(RpType::Any)
            },

            (_: array_type, _: bracket_start, argument: process_type_spec(), _: bracket_end) => {
                let argument = argument?;
                Ok(RpType::Array(Box::new(argument)))
            },

            (
                _: map_type,
                _: left_curly,
                key: process_type_spec(),
                _: colon,
                value: process_type_spec(),
                _: right_curly
             ) => {
                let key = key?;
                let value = value?;
                Ok(RpType::Map(Box::new(key), Box::new(value)))
            },

            (_: custom_type, name: process_name()) => {
                Ok(RpType::Name(name))
            },
        }

        process_name(&self) -> RpName {
            (prefix: process_used_prefix(), parts: type_identifier_list()) => {
                RpName {
                    prefix: prefix,
                    parts: parts,
                }
            },
        }

        process_modifier(&self) -> RpModifier {
            (_: optional) => RpModifier::Optional,
            () => RpModifier::Required,
        }
    }
}

/// Extend Rdp with helper utilities.
impl<'input, T: Input<'input>> Rdp<T> {
    pub fn match_member_list(&self) -> Result<Vec<AstLoc<MatchMember>>> {
        self.result_list_loc(Rule::match_member, None, Rdp::process_match_member)
    }

    pub fn sub_type_list(&self) -> Result<Vec<AstLoc<SubType>>> {
        self.result_list_loc(Rule::sub_type, None, Rdp::process_sub_type)
    }

    pub fn member_list(&self) -> Result<Vec<AstLoc<Member>>> {
        self.result_list_loc(Rule::member, None, Rdp::process_member)
    }

    pub fn enum_value_list(&self) -> Result<Vec<AstLoc<EnumValue>>> {
        self.result_list_loc(Rule::enum_value, None, Rdp::process_enum_value)
    }

    pub fn decl_list(&self) -> Result<Vec<AstLoc<Decl>>> {
        self.result_list_loc(Rule::decl, None, Rdp::process_decl)
    }

    pub fn value_list(&self) -> Result<Vec<AstLoc<Value>>> {
        self.result_list_loc(Rule::value, Some(Rule::comma), Rdp::process_value)
    }

    pub fn field_init_list(&self) -> Result<Vec<AstLoc<FieldInit>>> {
        self.result_list_loc(Rule::field_init, Some(Rule::comma), Rdp::process_field_init)
    }

    pub fn use_decl_list(&self) -> Vec<AstLoc<UseDecl>> {
        self.list(Rule::use_decl,
                  None,
                  |token| (self.process_use_decl(), *token).into())
    }

    pub fn identifier_list(&self) -> Vec<String> {
        self.list(Rule::identifier,
                  Some(Rule::dot),
                  |t| self.input().slice(t.start, t.end).to_owned())
    }

    pub fn type_identifier_list(&self) -> Vec<String> {
        self.list(Rule::type_identifier,
                  Some(Rule::dot),
                  |t| self.input().slice(t.start, t.end).to_owned())
    }

    pub fn result_list_loc<O, F>(&self,
                                 marker: Rule,
                                 sep: Option<Rule>,
                                 rule: F)
                                 -> Result<Vec<AstLoc<O>>>
        where F: Fn(&Self) -> Result<O>
    {
        self.result_list(marker,
                         sep,
                         |token| rule(self).map(move |item| (item, *token).into()))
    }

    pub fn result_list<O, F>(&self, marker: Rule, sep: Option<Rule>, rule: F) -> Result<Vec<O>>
        where F: Fn(&Token<Rule>) -> Result<O>
    {
        let mut out = Vec::new();

        for n in self.list(marker, sep, rule) {
            out.push(n?);
        }

        Ok(out)
    }

    pub fn list<O, F>(&self, marker: Rule, sep: Option<Rule>, rule: F) -> Vec<O>
        where F: Fn(&Token<Rule>) -> O
    {
        let mut list = Vec::new();

        while let Some(token) = self.queue().get(self.queue_index()) {
            if token.rule != marker {
                break;
            }

            self.inc_queue_index();
            list.push(rule(token));

            if let Some(sep) = sep {
                if let Some(token) = self.queue().get(self.queue_index()) {
                    if token.rule == sep {
                        self.inc_queue_index();
                        continue;
                    }
                }

                break;
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check that a parsed value equals expected.
    macro_rules! assert_value_eq {
        ($expected:expr, $input:expr) => {{
            let mut parser = parse($input);
            assert!(parser.value());

            let v = parser.process_value_token().unwrap().inner;
            assert_eq!($expected, v);
        }}
    }

    macro_rules! assert_type_spec_eq {
        ($expected:expr, $input:expr) => {{
            let mut parser = parse($input);
            assert!(parser.type_spec());
            assert!(parser.end());

            let v = parser.process_type_spec().unwrap();
            assert_eq!($expected, v);
        }}
    }

    const FILE1: &[u8] = include_bytes!("tests/file1.reproto");
    const INTERFACE1: &[u8] = include_bytes!("tests/interface1.reproto");

    fn parse(input: &'static str) -> Rdp<StringInput> {
        Rdp::new(StringInput::new(input))
    }

    #[test]
    fn test_file1() {
        let input = ::std::str::from_utf8(FILE1).unwrap();
        let mut parser = parse(input);

        assert!(parser.file());
        assert!(parser.end());

        let file = parser.process_file().unwrap();

        let package = RpPackage::new(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]);

        assert_eq!(package, *file.package);
        assert_eq!(4, file.decls.len());
    }

    #[test]
    fn test_array() {
        let mut parser = parse("[string]");

        assert!(parser.type_spec());
        assert!(parser.end());

        let ty = parser.process_type_spec().unwrap();

        if let RpType::Array(inner) = ty {
            if let RpType::String = *inner {
                return;
            }
        }

        panic!("Expected Type::Array(Type::String)");
    }

    #[test]
    fn test_map() {
        let mut parser = parse("{string: unsigned/123}");

        assert!(parser.type_spec());
        assert!(parser.end());

        let ty = parser.process_type_spec().unwrap();

        // TODO: use #![feature(box_patterns)]:
        // if let Type::Map(box Type::String, box Type::Unsigned(size)) = ty {
        // }
        if let RpType::Map(key, value) = ty {
            if let RpType::String = *key {
                if let RpType::Unsigned(size) = *value {
                    assert_eq!(Some(123usize), size);
                    return;
                }
            }
        }

        panic!("Expected Type::Array(Type::String)");
    }

    #[test]
    fn test_block_comment() {
        let mut parser = parse("/* hello \n world */");

        assert!(parser.comment());
    }

    #[test]
    fn test_line_comment() {
        let mut parser = parse("// hello world\n");

        assert!(parser.comment());
    }

    #[test]
    fn test_code_block() {
        let mut parser = parse("a { b { c } d } e");

        assert!(parser.code_body());
        assert!(parser.end());
    }

    #[test]
    fn test_code() {
        let mut parser = parse("java{{\na { b { c } d } e\n}}");

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
    fn test_interface() {
        let input = ::std::str::from_utf8(INTERFACE1).unwrap();
        let mut parser = parse(input);

        assert!(parser.file());
        assert!(parser.end());

        let file = parser.process_file().unwrap();

        assert_eq!(1, file.decls.len());
    }

    #[test]
    fn test_instance() {
        let c = RpName {
            prefix: None,
            parts: vec!["Foo".to_owned(), "Bar".to_owned()],
        };

        let field = FieldInit {
            name: AstLoc::new("hello".to_owned(), (8, 13)),
            value: AstLoc::new(Value::Number(12.into()), (15, 17)),
        };

        let field = AstLoc::new(field, (8, 17));

        let instance = Instance {
            ty: c,
            arguments: AstLoc::new(vec![field], (7, 18)),
        };

        assert_value_eq!(Value::Instance(AstLoc::new(instance, (0, 18))),
                         "Foo.Bar(hello: 12)");
    }

    #[test]
    fn test_values() {
        assert_value_eq!(Value::String("foo\nbar".to_owned()), "\"foo\\nbar\"");

        assert_value_eq!(Value::Number(1.into()), "1");

        assert_value_eq!(Value::Number(RpNumber {
                             whole: 1.into(),
                             fraction: Some(25.into()),
                             exponent: None,
                         }),
                         "1.25");
    }

    #[test]
    fn test_type_spec() {
        let c = RpName {
            prefix: None,
            parts: vec!["Hello".to_owned(), "World".to_owned()],
        };

        assert_type_spec_eq!(RpType::String, "string");
        assert_type_spec_eq!(RpType::Name(c), "Hello.World");
    }

    #[test]
    fn test_option_decl() {
        let mut parser = parse("foo_bar_baz true, foo, \"bar\", 12;");

        assert!(parser.option_decl());
        assert!(parser.end());

        if let Member::Option(option) = parser.process_member().unwrap() {
            assert_eq!("foo_bar_baz", option.name);
            assert_eq!(4, option.values.len());

            assert_eq!(Value::Boolean(true), option.values[0].inner);
            assert_eq!(Value::Identifier("foo".to_owned()), option.values[1].inner);
            assert_eq!(Value::String("bar".to_owned()), option.values[2].inner);
            assert_eq!(Value::Number(12u32.into()), option.values[3].inner);
            return;
        }

        panic!("option did not match");
    }
}
