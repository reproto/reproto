#![recursion_limit = "80"]

error_chain! {
    foreign_links {
        ParseInt(::std::num::ParseIntError);
    }

    errors {
        InvalidEscape {
        }

        Syntax(s: String) {
            description("syntax error")
            display("syntax error: {}", s)
        }
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

use pest::prelude::*;

impl_rdp! {
    grammar! {
        json = { string ~ eoi }

        string  = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        escape  =  { ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["n"] | ["r"] | ["t"] | unicode) }
        unicode =  { ["u"] ~ hex ~ hex ~ hex ~ hex }
        hex     =  { ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }

        number = @{ ["-"]? ~ int ~ (["."] ~ ['0'..'9']+ ~ exp? | exp)? }
        int    =  { ["0"] | ['1'..'9'] ~ ['0'..'9']* }
        exp    =  { (["E"] | ["e"]) ~ (["+"] | ["-"])? ~ int }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }

    process! {
        process_string(&self) -> Result<String> {
            (&string: string) => decode_escaped_string(string)
        }
    }
}

#[test]
fn test_string() {
    let mut parser = Rdp::new(StringInput::new("\"hello\\nworld\\u1234\""));
    parser.string();
    assert_eq!("hello\nworld\u{1234}".to_owned(),
               parser.process_string().unwrap());
}
