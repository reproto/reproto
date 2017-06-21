use num::Zero;
use num::bigint::BigInt;
use reproto_core::RpNumber;
use std::str::CharIndices;
use super::errors::*;
use super::token::*;

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
    n0: Option<Option<(usize, char)>>,
    n1: Option<Option<(usize, char)>>,
    last_comment: Vec<&'input str>,
    buffer: String,
    illegal: bool,
    code_block: Option<(usize, usize)>,
    code_close: Option<(usize, usize)>,
}

impl<'input> Lexer<'input> {
    /// Advance the source iterator.
    #[inline]
    fn step(&mut self) {
        // shift
        if let Some(n1) = self.n1 {
            self.n0 = Some(n1);
            self.n1 = Some(self.source.next());
        } else {
            self.n0 = Some(self.source.next());
            self.n1 = Some(self.source.next());
        }
    }

    #[inline]
    fn step_n(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.step();
        }

        self.n0.and_then(|n| n).map(|n| n.0).unwrap_or_else(|| self.source_str.len())
    }

    #[inline]
    fn one(&mut self) -> Option<(usize, char)> {
        if self.n0.is_none() {
            self.step();
        }

        match self.n0 {
            Some(n0) => n0,
            None => None,
        }
    }

    #[inline]
    fn two(&mut self) -> Option<(usize, char, char)> {
        if self.n0.is_none() {
            self.step();
        }

        if let (Some((pos, a)), Some((_, b))) = (self.n0.unwrap_or(None), self.n1.unwrap_or(None)) {
            Some((pos, a, b))
        } else {
            None
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.n0.and_then(|n| n).map(|n| n.0).unwrap_or(0usize)
    }

    fn identifier(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        // strip leading _
        let (stripped, _) = take!(self, start, '_');
        let (end, content) = take!(self, start, 'a'...'z' | '_' | '0'...'9');

        if stripped != start {
            let identifier = commented(self.last_comment.clone(), content);
            self.last_comment.clear();
            let token = Token::Identifier(identifier);
            return Ok((start, token, end));
        }

        let token = match content {
            "any" => Token::AnyKeyword,
            "on" => Token::OnKeyword,
            "interface" => Token::InterfaceKeyword,
            "type" => Token::TypeKeyword,
            "enum" => Token::EnumKeyword,
            "tuple" => Token::TupleKeyword,
            "service" => Token::ServiceKeyword,
            "package" => Token::PackageKeyword,
            "match" => Token::MatchKeyword,
            "use" => Token::UseKeyword,
            "as" => Token::AsKeyword,
            "float" => Token::FloatKeyword,
            "double" => Token::DoubleKeyword,
            "signed" => Token::SignedKeyword,
            "unsigned" => Token::UnsignedKeyword,
            "boolean" => Token::BooleanKeyword,
            "string" => Token::StringKeyword,
            "bytes" => Token::BytesKeyword,
            "true" => Token::TrueKeyword,
            "false" => Token::FalseKeyword,
            "endpoint" => {
                let comment = self.last_comment.clone();
                self.last_comment.clear();
                Token::EndpointKeyword(comment)
            }
            "returns" => {
                let comment = self.last_comment.clone();
                self.last_comment.clear();
                Token::ReturnsKeyword(comment)
            }
            identifier => {
                let identifier = commented(self.last_comment.clone(), identifier);
                self.last_comment.clear();
                let token = Token::Identifier(identifier);
                return Ok((start, token, end));
            }
        };

        return Ok((start, token, end));
    }

    fn type_identifier(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        let (end, content) = take!(self, start, 'A'...'Z' | 'a'...'z' | '_' | '0'...'9');
        let type_identifier = commented(self.last_comment.clone(), content);
        self.last_comment.clear();
        Ok((start, Token::TypeIdentifier(type_identifier), end))
    }

    fn parse_fraction(input: &str) -> Result<(usize, BigInt)> {
        let dec = input.chars()
            .enumerate()
            .find(|&(_, ref c)| *c != '0')
            .map(|(i, _)| i)
            .unwrap_or(0usize);

        let fraction: BigInt = input.parse()?;

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
        let (negative, offset) = if let Some((_, '-')) = self.one() {
            (true, self.step_n(1))
        } else {
            (false, start)
        };

        let (mut end, mut digits) = {
            let (end, whole) = take!(self, offset, '0'...'9');
            (end, whole.parse::<BigInt>()?)
        };

        let mut decimal = 0usize;

        if let Some((_, '.')) = self.one() {
            let offset = self.step_n(1);

            {
                let (e, fraction) = take!(self, offset, '0'...'9');
                end = e;
                let (dec, fraction) = Self::parse_fraction(fraction)?;
                Self::apply_fraction(&mut digits, &mut decimal, dec, fraction);
            }

            if let Some((_, 'e')) = self.one() {
                let offset = self.step_n(1);

                let (e, content) = take!(self, offset, '-' | '0'...'9');
                end = e;
                let exponent: i32 = content.parse()?;
                Self::apply_exponent(&mut digits, &mut decimal, exponent);
            }
        }

        let digits = if negative { -digits } else { digits };

        let number = RpNumber {
            digits: digits,
            decimal: decimal,
        };

        self.last_comment.clear();
        Ok((start, Token::Number(number), end))
    }

    fn decode_unicode4(&mut self) -> Result<char> {
        let mut res = 0u32;

        for x in 0..4u32 {
            let c = self.one().map(|(_, c)| c).ok_or("expected hex character")?.to_string();
            let c = u32::from_str_radix(&c, 16)?;
            res += c << (4 * (3 - x));
            self.step();
        }

        Ok(::std::char::from_u32(res).ok_or("expected valid character")?)
    }

    /// Tokenize string.
    fn string(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        self.buffer.clear();

        self.step();

        while let Some((_, c)) = self.one() {
            if c == '\\' {
                self.step();

                if let Some((_, escape)) = self.one() {
                    let escaped = match escape {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'u' => self.decode_unicode4()?,
                        _ => break,
                    };

                    self.step();
                    self.buffer.push(escaped);
                    continue;
                }

                break;
            }

            if c == '"' {
                let end = self.step_n(1);
                self.last_comment.clear();
                return Ok((start, Token::String(self.buffer.clone()), end));
            }

            self.buffer.push(c);
            self.step();
        }

        self.illegal()
    }

    /// Tokenize code block.
    /// TODO: support escape sequences for languages where `}}` might occur.
    fn code_block(&mut self,
                  code_start: usize,
                  start: usize)
                  -> Result<(usize, Token<'input>, usize)> {
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

        self.illegal()
    }

    fn line_comment(&mut self) {
        let start = self.step_n(2);
        let mut end = start;

        while let Some((e, c)) = self.one() {
            match c {
                '\n' | '\r' => {
                    end = e;
                    self.step();
                    break;
                }
                _ => {
                    end = e;
                    self.step();
                }
            }
        }

        self.last_comment.push(&self.source_str[start..end]);
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

    fn version(&mut self) -> Result<(usize, Token<'input>, usize)> {
        let start = self.step_n(1);
        let (start, _) = take!(self, start, ' ' | '\n' | '\r' | '\t');

        let (end, content) =
            take!(self, start, '^' | '<' | '>' | '=' | '.' | '-' | '0'...'9' | 'a'...'z');

        return Ok((start, Token::Version(content.to_owned()), end));
    }

    fn illegal<T>(&mut self) -> Result<T> {
        self.illegal = true;
        Err(ErrorKind::IllegalToken(self.pos()).into())
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.illegal {
            return Some(self.illegal());
        }

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
                        self.last_comment.clear();
                        return Some(Ok((start, Token::CodeOpen, end)));
                    }
                    (':', ':') => Some(Token::Scope),
                    ('=', '>') => Some(Token::HashRocket),
                    _ => None,
                };

                if let Some(token) = token {
                    let end = self.step_n(2);
                    self.last_comment.clear();
                    return Some(Ok((start, token, end)));
                }
            }

            // one character keywords
            if let Some((start, c)) = self.one() {
                let token = match c {
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
                    '?' => Token::Optional,
                    '&' => Token::And,
                    '/' => Token::Slash,
                    '=' => Token::Equals,
                    '*' => {
                        let comment = self.last_comment.clone();
                        self.last_comment.clear();
                        Token::Star(comment)
                    }
                    '@' => {
                        return Some(self.version());
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

                self.last_comment.clear();
                self.step();
                return Some(Ok((start, token, start + 1)));
            } else {
                return None;
            }
        }

        Some(self.illegal())
    }
}

pub fn lex(input: &str) -> Lexer {
    Lexer {
        source: input.char_indices(),
        source_len: input.len(),
        source_str: input,
        n0: None,
        n1: None,
        last_comment: Vec::new(),
        buffer: String::new(),
        illegal: false,
        code_block: None,
        code_close: None,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::Token::*;

    pub fn empty<'input, T>(value: T) -> Commented<'input, T> {
        Commented {
            comment: vec![],
            value: value,
        }
    }

    fn tokenize(input: &str) -> Result<Vec<(usize, Token, usize)>> {
        lex(input).collect()
    }

    #[test]
    pub fn test_lexer() {
        let expected = vec![(0, Identifier(empty("hello")), 5),
                            (6, TypeIdentifier(empty("World")), 11),
                            (12, LeftCurly, 13),
                            (14, UseKeyword, 17),
                            (18, AsKeyword, 20),
                            (21, RightCurly, 22),
                            (23, String("hello world".into()), 36)];

        assert_eq!(expected,
                   tokenize("hello World { use as } \"hello world\"").unwrap());
    }

    #[test]
    pub fn test_code_block() {
        let expected = vec![(0, CodeOpen, 2),
                            (0, CodeContent(" foo bar baz \n zing ".into()), 24),
                            (22, CodeClose, 24)];

        assert_eq!(expected, tokenize("{{ foo bar baz \n zing }}").unwrap());
    }

    #[test]
    pub fn test_complex_number() {
        let expected = vec![(0,
                             Number(RpNumber {
                                 digits: (-1242).into(),
                                 decimal: 6,
                             }),
                             9)];

        assert_eq!(expected, tokenize("-12.42e-4").unwrap());
    }

    #[test]
    pub fn test_number_2() {
        assert_eq!(vec![(0, Number(12.into()), 2)], tokenize("12").unwrap());
    }

    #[test]
    pub fn test_name() {
        let expected = vec![(0, Identifier(empty("foo")), 3),
                            (3, Scope, 5),
                            (5, TypeIdentifier(empty("Bar")), 8),
                            (8, Dot, 9),
                            (9, TypeIdentifier(empty("Baz")), 12)];

        assert_eq!(expected, tokenize("foo::Bar.Baz").unwrap());
    }

    #[test]
    pub fn test_instance() {
        let expected = vec![(0, Identifier(empty("foo")), 3),
                            (3, Scope, 5),
                            (5, TypeIdentifier(empty("Bar")), 8),
                            (8, Dot, 9),
                            (9, TypeIdentifier(empty("Baz")), 12),
                            (12, LeftParen, 13),
                            (13, Identifier(empty("hello")), 18),
                            (18, Colon, 19),
                            (20, Number(12.into()), 22),
                            (22, RightParen, 23)];

        assert_eq!(expected, tokenize("foo::Bar.Baz(hello: 12)").unwrap());
    }

    #[test]
    pub fn test_comments() {
        let tokens = tokenize("// hello \n world");
        let comment = vec![" hello "];
        assert_eq!(vec![(11, Identifier(commented(comment, "world")), 16)],
                   tokens.unwrap());

        let tokens = tokenize("he/* this is a comment */llo");
        assert_eq!(vec![(0, Identifier(empty("he")), 2), (25, Identifier(empty("llo")), 28)],
                   tokens.unwrap());

        let tokens = tokenize("// test\n// this\nhello");
        let comment = vec![" test", " this"];

        assert_eq!(vec![(16, Identifier(commented(comment, "hello")), 21)],
                   tokens.unwrap());
    }

    #[test]
    pub fn test_version_req() {
        let tokens = tokenize("@>=1.0");
        println!("tokens = {:?}", tokens);
    }
}
