use super::token::*;
use core::{RpNumber, VersionReq};
use num::Zero;
use num::bigint::BigInt;
use std::result;
use std::str::CharIndices;

/// Take until the givern pattern matches.
/// This will return content up-until the pattern matches, and consume the pattern itself.
macro_rules! take_until {
    ($slf:expr, $start:expr, $first:pat $(| $rest:pat)*) => {{
        let mut __end = $start;
        let mut __content_end = $start;

        loop {
            if let Some((e, c)) = $slf.one() {
                __content_end = e;

                match c {
                    $first $(| $rest)* => {
                        let (e, _) = take!($slf, e, $first $(| $rest)*);
                        __end = e;
                        break;
                    }
                    _ => {
                        __end = e;
                        $slf.step();
                    }
                }
            } else {
                __content_end = $slf.source_len;
                break;
            }
        }

        (__end, &$slf.source_str[$start..__content_end])
    }}
}

/// Take while pattern matches.
macro_rules! take {
    ($slf:expr, $start:expr, $first:pat $(| $rest:pat)*) => {{
        let mut __end: usize = $start;

        loop {
            if let Some((__pos, __c)) = $slf.one() {
                __end = __pos;

                match __c {
                    $first $(| $rest)* => {},
                    _ => break,
                }

                $slf.step();
            } else {
                __end = $slf.source_len;
                break;
            }
        }

        (__end, &$slf.source_str[$start..__end])
    }}
}

pub struct Lexer<'input> {
    source: CharIndices<'input>,
    source_len: usize,
    source_str: &'input str,
    n0: Option<(usize, char)>,
    n1: Option<(usize, char)>,
    n2: Option<(usize, char)>,
    buffer: String,
    code_block: Option<(usize, usize)>,
    code_close: Option<(usize, usize)>,
    path_mode: bool,
    path_variable_nesting: usize,
    path_buffer: String,
    version_req_mode: bool,
}

impl<'input> Lexer<'input> {
    /// Advance the source iterator.
    #[inline]
    fn step(&mut self) {
        self.n0 = self.n1;
        self.n1 = self.n2;
        self.n2 = self.source.next();
    }

    #[inline]
    fn step_n(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.step();
        }

        self.n0.map(|n| n.0).unwrap_or_else(
            || self.source_str.len(),
        )
    }

    #[inline]
    fn one(&mut self) -> Option<(usize, char)> {
        self.n0
    }

    #[inline]
    fn two(&mut self) -> Option<(usize, char, char)> {
        if let (Some((pos, a)), Some((_, b))) = (self.n0, self.n1) {
            Some((pos, a, b))
        } else {
            None
        }
    }

    #[inline]
    fn three(&mut self) -> Option<(usize, char, char, char)> {
        if let (Some((pos, a)), Some((_, b)), Some((_, c))) = (self.n0, self.n1, self.n2) {
            Some((pos, a, b, c))
        } else {
            None
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.n0.map(|n| n.0).unwrap_or(self.source_len)
    }

    fn identifier(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        // strip leading _
        let (stripped, _) = take!(self, start, '_');
        let (end, content) = take!(self, stripped, 'a'...'z' | '_' | '0'...'9');

        if stripped != start {
            return Ok((start, Token::Identifier(content), end));
        }

        let token = match content {
            "any" => Token::AnyKeyword,
            "interface" => Token::InterfaceKeyword,
            "type" => Token::TypeKeyword,
            "enum" => Token::EnumKeyword,
            "tuple" => Token::TupleKeyword,
            "service" => Token::ServiceKeyword,
            "use" => Token::UseKeyword,
            "as" => Token::AsKeyword,
            "float" => Token::FloatKeyword,
            "double" => Token::DoubleKeyword,
            "signed" => Token::SignedKeyword,
            "unsigned" => Token::UnsignedKeyword,
            "boolean" => Token::BooleanKeyword,
            "string" => Token::StringKeyword,
            "datetime" => Token::DateTimeKeyword,
            "bytes" => Token::BytesKeyword,
            "true" => Token::TrueKeyword,
            "false" => Token::FalseKeyword,
            "returns" => Token::ReturnsKeyword,
            "accepts" => Token::AcceptsKeyword,
            identifier => {
                return Ok((start, Token::Identifier(identifier), end));
            }
        };

        return Ok((start, token, end));
    }

    fn type_identifier(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        let (end, content) = take!(self, start, 'A'...'Z' | 'a'...'z' | '_' | '0'...'9');
        Ok((start, Token::TypeIdentifier(content), end))
    }

    fn parse_fraction(input: &str) -> result::Result<(usize, BigInt), &'static str> {
        let dec = input
            .chars()
            .enumerate()
            .find(|&(_, ref c)| *c != '0')
            .map(|(i, _)| i)
            .unwrap_or(0usize);

        let fraction: BigInt = input.parse().map_err(|_| "illegal fraction")?;

        Ok((dec, fraction))
    }

    fn apply_fraction(digits: &mut BigInt, decimal: &mut usize, dec: usize, fraction: BigInt) {
        *decimal += dec;

        let mut f = fraction.clone();
        let ten: BigInt = 10.into();

        while !f.is_zero() {
            *digits = digits.clone() * ten.clone();
            *decimal += 1;
            f = f / ten.clone();
        }

        *digits = digits.clone() + fraction;
    }

    fn apply_exponent(digits: &mut BigInt, decimal: &mut usize, exponent: i32) {
        if exponent < 0 {
            *decimal += exponent.abs() as usize;
            return;
        }

        let ten: BigInt = 10.into();

        for _ in 0..exponent {
            if *decimal > 0 {
                *decimal = *decimal - 1;
            } else {
                *digits = digits.clone() * ten.clone();
            }
        }
    }

    fn number(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        let (end, number) = self.parse_number(start).map_err(|(message, offset)| {
            Error::InvalidNumber {
                message: message,
                pos: start + offset,
            }
        })?;

        Ok((start, Token::Number(number), end))
    }

    fn parse_number(
        &mut self,
        start: usize,
    ) -> result::Result<(usize, RpNumber), (&'static str, usize)> {
        let (negative, offset) = if let Some((_, '-')) = self.one() {
            (true, self.step_n(1))
        } else {
            (false, start)
        };

        let (mut end, mut digits) = {
            let (end, whole) = take!(self, offset, '0'...'9');
            (
                end,
                whole.parse::<BigInt>().map_err(|_| ("illegal number", end))?,
            )
        };

        let mut decimal = 0usize;

        if let Some((_, '.')) = self.one() {
            let offset = self.step_n(1);

            {
                let (e, fraction) = take!(self, offset, '0'...'9');
                end = e;
                let (dec, fraction) = Self::parse_fraction(fraction).map_err(|e| (e, end))?;
                Self::apply_fraction(&mut digits, &mut decimal, dec, fraction);
            }

            if let Some((_, 'e')) = self.one() {
                let offset = self.step_n(1);

                let (e, content) = take!(self, offset, '-' | '0'...'9');
                end = e;
                let exponent: i32 = content.parse().map_err(|_| ("illegal exponent", end))?;
                Self::apply_exponent(&mut digits, &mut decimal, exponent);
            }
        }

        let digits = if negative { -digits } else { digits };

        let number = RpNumber {
            digits: digits,
            decimal: decimal,
        };

        Ok((end, number))
    }

    // decode a sequence of 4 unicode characters
    fn decode_unicode4(&mut self) -> result::Result<char, (&'static str, usize)> {
        let mut res = 0u32;

        for x in 0..4u32 {
            let c = self.one()
                .ok_or_else(|| ("expected digit", x as usize))?
                .1
                .to_string();
            let c = u32::from_str_radix(&c, 16).map_err(|_| {
                ("expected hex digit", x as usize)
            })?;
            res += c << (4 * (3 - x));
            self.step();
        }

        Ok(::std::char::from_u32(res).ok_or_else(
            || ("invalid character", 0usize),
        )?)
    }

    fn escape(&mut self, pos: usize) -> Result<char> {
        self.step();

        let (_, escape) = self.one().ok_or_else(
            || Error::UnterminatedEscape { start: self.pos() },
        )?;

        let escaped = match escape {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '{' => '{',
            '}' => '}',
            '/' => '/',
            '?' => '?',
            '&' => '&',
            '=' => '=',
            'u' => {
                let seq_start = self.step_n(1);

                let c = self.decode_unicode4().map_err(|(message, offset)| {
                    Error::InvalidEscape {
                        message: message,
                        pos: seq_start + offset,
                    }
                })?;

                return Ok(c);
            }
            _ => {
                return Err(
                    Error::InvalidEscape {
                        message: "unrecognized escape, should be one of: \\n, \\r, \\t, or \\uXXXX",
                        pos: pos,
                    }.into(),
                );
            }
        };

        self.step();
        return Ok(escaped);
    }

    /// Tokenize string.
    fn string(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        self.buffer.clear();

        self.step();

        while let Some((pos, c)) = self.one() {
            if c == '\\' {
                let c = self.escape(pos)?;
                self.buffer.push(c);
                continue;
            }

            if c == '"' {
                let end = self.step_n(1);
                return Ok((start, Token::String(self.buffer.clone()), end));
            }

            self.buffer.push(c);
            self.step();
        }

        Err(Error::UnterminatedString { start: start }.into())
    }

    /// Tokenize code block.
    /// TODO: support escape sequences for languages where `}}` might occur.
    fn code_block(
        &mut self,
        code_start: usize,
        start: usize,
    ) -> Result<(usize, Token<'input>, usize)> {
        while let Some((end, a, b)) = self.two() {
            if ('}', '}') == (a, b) {
                let code_end = self.step_n(2);
                let out = &self.source_str[start..end];

                // emit code end at next iteration.
                self.code_block = None;
                self.code_close = Some((end, code_end));

                return Ok((code_start, Token::CodeContent(out), code_end));
            }

            self.step();
        }

        Err(Error::UnterminatedCodeBlock { start: start }.into())
    }

    fn doc_comments(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        let mut comment: Vec<&'input str> = Vec::new();

        loop {
            // take leading whitespace
            let (end, _) = take!(self, start, ' ' | '\n' | '\r' | '\t');

            if let Some((_, '/', '/', '/')) = self.three() {
                let start = self.step_n(3);
                let (_, content) = take_until!(self, start, '\n' | '\r');
                comment.push(content);
            } else {
                return Ok((start, Token::DocComment(comment), end));
            }
        }
    }

    fn line_comment(&mut self) {
        let start = self.step_n(2);
        let _ = take_until!(self, start, '\n' | '\r');
    }

    // block comments have no semantics and are completely ignored.
    fn block_comment(&mut self) {
        self.step_n(2);

        while let Some((_, a, b)) = self.two() {
            if ('*', '/') == (a, b) {
                self.step();
                self.step();
                break;
            }

            self.step();
        }
    }

    fn version_req(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        let (end, content) =
            take!(self, start, '^' | '<' | '>' | '=' | '.' | '-' | '0'...'9' | 'a'...'z');

        let version_req = VersionReq::parse(content).map_err(|_| {
            Error::InvalidVersionReq {
                start: start,
                end: end,
            }
        })?;

        Ok((start, Token::VersionReq(version_req), end))
    }

    fn version_req_next(&mut self) -> Option<Result<(usize, Token<'input>, usize)>> {
        let (start, _) = take!(self, self.pos(), ' ' | '\n' | '\r' | '\t');
        let version_req = self.version_req(start);
        self.version_req_mode = false;
        return Some(version_req);
    }

    fn path_variable_mode_next(&mut self) -> Option<Result<(usize, Token<'input>, usize)>> {
        loop {
            if let Some((start, c)) = self.one() {
                let token = match c {
                    ':' => Token::Colon,
                    '{' => {
                        self.path_variable_nesting += 1;
                        Token::LeftCurly
                    }
                    '}' => {
                        self.path_variable_nesting -= 1;
                        Token::RightCurly
                    }
                    ' ' | '\n' | '\r' | '\t' => {
                        self.step();
                        continue;
                    }
                    '_' | 'a'...'z' => return Some(self.identifier(start)),
                    _ => break,
                };

                let end = self.step_n(1);
                return Some(Ok((start, token, end)));
            } else {
                return None;
            }
        }

        Some(Err(Error::Unexpected { pos: self.pos() }))
    }

    fn take_path_buffer(&mut self) -> Option<String> {
        if self.path_buffer.is_empty() {
            return None;
        }

        let buffer = self.path_buffer.clone();
        self.path_buffer.clear();
        return Some(buffer);
    }

    fn path_mode_next(&mut self) -> Option<Result<(usize, Token<'input>, usize)>> {
        while let Some((pos, c)) = self.one() {
            let token = match c {
                '\\' => {
                    let c = match self.escape(pos) {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };

                    self.path_buffer.push(c);
                    continue;
                }
                '{' => {
                    if let Some(buffer) = self.take_path_buffer() {
                        return Some(Ok((pos - buffer.len(), Token::PathSegment(buffer), pos)));
                    }

                    self.path_variable_nesting = 1;
                    Token::LeftCurly
                }
                '`' => {
                    if let Some(buffer) = self.take_path_buffer() {
                        return Some(Ok((pos - buffer.len(), Token::PathSegment(buffer), pos)));
                    }

                    self.path_mode = false;
                    Token::Tick
                }
                '/' => {
                    if let Some(buffer) = self.take_path_buffer() {
                        return Some(Ok((pos - buffer.len(), Token::PathSegment(buffer), pos)));
                    }

                    Token::Slash
                }
                '?' => Token::QuestionMark,
                '&' => Token::And,
                '=' => Token::Equals,
                c => {
                    self.path_buffer.push(c);
                    self.step();
                    continue;
                }
            };

            let end = self.step_n(1);
            return Some(Ok((pos, token, end)));
        }

        Some(Err(Error::Unexpected { pos: self.pos() }))
    }

    fn normal_mode_next(&mut self) -> Option<Result<(usize, Token<'input>, usize)>> {
        // dispatch a CodeClose.
        if let Some((start, end)) = self.code_close {
            self.code_close = None;
            return Some(Ok((start, Token::CodeClose, end)));
        }

        // code block mode
        if let Some((code_start, start)) = self.code_block {
            return Some(self.code_block(code_start, start));
        }

        loop {
            // doc comments
            if let Some((start, '/', '/', '/')) = self.three() {
                return Some(self.doc_comments(start));
            }

            // two character keywords
            if let Some((start, a, b)) = self.two() {
                let token = match (a, b) {
                    ('/', '/') => {
                        self.line_comment();
                        continue;
                    }
                    ('/', '*') => {
                        self.block_comment();
                        continue;
                    }
                    ('{', '{') => {
                        let end = self.step_n(2);
                        self.code_block = Some((start, end));
                        return Some(Ok((start, Token::CodeOpen, end)));
                    }
                    (':', ':') => Some(Token::Scope),
                    ('=', '>') => Some(Token::HashRocket),
                    _ => None,
                };

                if let Some(token) = token {
                    let end = self.step_n(2);
                    return Some(Ok((start, token, end)));
                }
            }

            // one character keywords
            if let Some((start, c)) = self.one() {
                let token = match c {
                    '`' => {
                        self.path_mode = true;
                        self.path_buffer.clear();
                        Token::Tick
                    }
                    '{' => Token::LeftCurly,
                    '}' => Token::RightCurly,
                    '[' => Token::LeftBracket,
                    ']' => Token::RightBracket,
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    ';' => Token::SemiColon,
                    ':' => Token::Colon,
                    ',' => Token::Comma,
                    '.' => Token::Dot,
                    '?' => Token::QuestionMark,
                    '&' => Token::And,
                    '/' => Token::Slash,
                    '=' => Token::Equals,
                    '*' => Token::Star,
                    '@' => {
                        self.version_req_mode = true;
                        Token::At
                    }
                    '_' | 'a'...'z' => return Some(self.identifier(start)),
                    'A'...'Z' => return Some(self.type_identifier(start)),
                    '"' => return Some(self.string(start)),
                    '-' | '0'...'9' => return Some(self.number(start)),
                    // ignore whitespace
                    ' ' | '\n' | '\r' | '\t' => {
                        self.step();
                        continue;
                    }
                    _ => break,
                };

                let end = self.step_n(1);
                return Some(Ok((start, token, end)));
            } else {
                return None;
            }
        }

        Some(Err(Error::Unexpected { pos: self.pos() }))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.version_req_mode {
            return self.version_req_next();
        }

        if self.path_variable_nesting > 0 {
            return self.path_variable_mode_next();
        }

        if self.path_mode {
            return self.path_mode_next();
        }

        self.normal_mode_next()
    }
}

pub fn lex(input: &str) -> Lexer {
    let mut source = input.char_indices();

    let n0 = source.next();
    let n1 = source.next();
    let n2 = source.next();

    Lexer {
        source: source,
        source_len: input.len(),
        source_str: input,
        n0: n0,
        n1: n1,
        n2: n2,
        buffer: String::new(),
        code_block: None,
        code_close: None,
        path_mode: false,
        path_variable_nesting: 0usize,
        path_buffer: String::new(),
        version_req_mode: false,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::Token::*;
    use core::VersionReq;

    fn tokenize(input: &str) -> Result<Vec<(usize, Token, usize)>> {
        lex(input).collect()
    }

    #[test]
    pub fn test_lexer() {
        let expected = vec![
            (0, Identifier("hello"), 5),
            (6, TypeIdentifier("World"), 11),
            (12, LeftCurly, 13),
            (14, UseKeyword, 17),
            (18, AsKeyword, 20),
            (21, RightCurly, 22),
            (23, String("hello world".into()), 36),
        ];

        assert_eq!(
            expected,
            tokenize("hello World { use as } \"hello world\"").unwrap()
        );
    }

    #[test]
    pub fn test_code_block() {
        let expected = vec![
            (0, CodeOpen, 2),
            (0, CodeContent(" foo bar baz \n zing ".into()), 24),
            (22, CodeClose, 24),
        ];

        assert_eq!(expected, tokenize("{{ foo bar baz \n zing }}").unwrap());
    }

    #[test]
    pub fn test_complex_number() {
        let expected = vec![
            (
                0,
                Number(RpNumber {
                    digits: (-1242).into(),
                    decimal: 6,
                }),
                9
            ),
        ];

        assert_eq!(expected, tokenize("-12.42e-4").unwrap());
    }

    #[test]
    pub fn test_number_2() {
        assert_eq!(vec![(0, Number(12.into()), 2)], tokenize("12").unwrap());
    }

    #[test]
    pub fn test_name() {
        let expected = vec![
            (0, Identifier("foo"), 3),
            (3, Scope, 5),
            (5, TypeIdentifier("Bar"), 8),
            (8, Dot, 9),
            (9, TypeIdentifier("Baz"), 12),
        ];

        assert_eq!(expected, tokenize("foo::Bar.Baz").unwrap());
    }

    #[test]
    pub fn test_strings() {
        let expected = vec![(0, String("foo\nbar".to_owned()), 10)];
        assert_eq!(expected, tokenize("\"foo\\nbar\"").unwrap());
    }

    #[test]
    pub fn test_instance() {
        let expected = vec![
            (0, Identifier("foo"), 3),
            (3, Scope, 5),
            (5, TypeIdentifier("Bar"), 8),
            (8, Dot, 9),
            (9, TypeIdentifier("Baz"), 12),
            (12, LeftParen, 13),
            (13, Identifier("hello"), 18),
            (18, Colon, 19),
            (20, Number(12.into()), 22),
            (22, RightParen, 23),
        ];

        assert_eq!(expected, tokenize("foo::Bar.Baz(hello: 12)").unwrap());
    }

    #[test]
    pub fn test_comments() {
        let tokens = tokenize("// hello \n world");
        assert_eq!(vec![(11, Identifier("world"), 16)], tokens.unwrap());

        let tokens = tokenize("he/* this is a comment */llo");
        assert_eq!(
            vec![(0, Identifier("he"), 2), (25, Identifier("llo"), 28)],
            tokens.unwrap()
        );

        let tokens = tokenize("// test\n// this\nhello");
        assert_eq!(vec![(16, Identifier("hello"), 21)], tokens.unwrap());
    }

    #[test]
    pub fn test_identifier_stripping() {
        let a = &tokenize("my_version").unwrap()[0].1;
        let b = &tokenize("_my_version").unwrap()[0].1;
        let c = &tokenize("__my_version").unwrap()[0].1;

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    pub fn test_doc_comment() {
        let tokens = tokenize("/// foo\n\r      /// bar \r\n     /// baz ").unwrap();
        let reference = [(0, DocComment(vec![" foo", " bar ", " baz "]), 38)];
        assert_eq!(reference, &tokens[..]);
    }

    #[test]
    pub fn test_path() {
        let tokens = tokenize("`/foo/first_\\/{id:{string: unsigned}}`").unwrap();

        let reference = [
            (0, Tick, 1),
            (1, Slash, 2),
            (2, PathSegment("foo".into()), 5),
            (5, Slash, 6),
            (7, PathSegment("first_/".into()), 14),
            (14, LeftCurly, 15),
            (15, Identifier("id"), 17),
            (17, Colon, 18),
            (18, LeftCurly, 19),
            (19, StringKeyword, 25),
            (25, Colon, 26),
            (27, UnsignedKeyword, 35),
            (35, RightCurly, 36),
            (36, RightCurly, 37),
            (37, Tick, 38),
        ];

        assert_eq!(reference, &tokens[..]);
    }

    #[test]
    pub fn test_version_req() {
        let tokens = tokenize("@>=1.0").unwrap();
        let version_req = VersionReq::parse(">=1.0").unwrap();
        let reference = [(0, At, 1), (1, VersionReq(version_req), 6)];
        assert_eq!(reference, &tokens[..]);
    }
}
