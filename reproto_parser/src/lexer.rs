use num::Zero;
use num::bigint::BigInt;
use reproto_core::RpNumber;
use super::errors::*;
use super::token::*;

macro_rules! take {
    ($slf:expr, $current:expr, $first:pat $(| $rest:pat)*) => {{
        let mut end: usize = $current;
        $slf.buffer.clear();

        while let Some((pos, c)) = $slf.one() {
            if let Some((_, '/', '*')) = $slf.two() {
                $slf.block_comment();
                continue;
            }

            end = pos;

            match c {
                $first $(| $rest)* => $slf.buffer.push(c),
                _ => break,
            }

            $slf.step();
        }

        (end, &$slf.buffer)
    }}
}

pub struct Lexer<I> {
    source: I,
    n0: Option<Option<(usize, char)>>,
    n1: Option<Option<(usize, char)>>,
    last_comment: Vec<String>,
    buffer: String,
    illegal: bool,
    code_block: Option<usize>,
}

impl<I> Lexer<I>
    where I: Iterator<Item = (usize, char)>
{
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

    fn identifier(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        // strip leading _
        let (stripped, _) = take!(self, start, '_');
        let (end, content) = take!(self, start, 'a'...'z' | '_' | '0'...'9');

        if stripped != start {
            let identifier = Commented::new(self.last_comment.clone(), content.to_owned());
            self.last_comment.clear();
            let token = Token::Identifier(identifier);
            return Ok((start, token, end));
        }

        let token = match content.as_str() {
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
                let identifier = Commented::new(self.last_comment.clone(), identifier.to_owned());
                self.last_comment.clear();
                let token = Token::Identifier(identifier);
                return Ok((start, token, end));
            }
        };

        return Ok((start, token, end));
    }

    fn type_identifier(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        let (end, content) = take!(self, start, 'A'...'Z' | 'a'...'z' | '_' | '0'...'9');
        let type_identifier = Commented::new(self.last_comment.clone(), content.to_owned());
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

    fn number(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        let negative = if let Some((_, '-')) = self.one() {
            self.step();
            true
        } else {
            false
        };

        let (mut end, mut digits) = {
            let (end, whole) = take!(self, start, '0'...'9');
            (end, whole.parse::<BigInt>()?)
        };

        let mut decimal = 0usize;

        if let Some((_, '.')) = self.one() {
            self.step();

            {
                let (e, fraction) = take!(self, end, '0'...'9');
                end = e;
                let (dec, fraction) = Self::parse_fraction(fraction)?;
                Self::apply_fraction(&mut digits, &mut decimal, dec, fraction);
            }

            if let Some((_, 'e')) = self.one() {
                self.step();

                let (e, content) = take!(self, end, '-' | '0'...'9');
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
    fn string(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        self.buffer.clear();

        self.step();

        while let Some((pos, c)) = self.one() {
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
                self.step();
                self.last_comment.clear();
                return Ok((start, Token::String(self.buffer.clone()), pos + 1));
            }

            self.buffer.push(c);
            self.step();
        }

        self.illegal()
    }

    /// Tokenize code block.
    /// TODO: support escape sequences for languages where `}}` might occur.
    fn code_block(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        self.buffer.clear();

        while let Some((_, c)) = self.one() {
            if let Some((pos, '}', '}')) = self.two() {
                self.code_block = None;
                return Ok((start, Token::CodeContent(self.buffer.clone()), pos));
            }

            self.buffer.push(c);
            self.step();
        }

        self.illegal()
    }

    fn line_comment(&mut self) {
        self.buffer.clear();

        self.step();
        self.step();

        while let Some((_, c)) = self.one() {
            match c {
                '\n' | '\r' => {
                    self.step();
                    break;
                }
                c => {
                    self.buffer.push(c);
                    self.step();
                }
            }
        }

        self.last_comment.push(self.buffer.clone());
    }

    // block comments have no semantics and are completely ignored.
    fn block_comment(&mut self) {
        self.step();
        self.step();

        while let Some((_, a, b)) = self.two() {
            if ('*', '/') == (a, b) {
                self.step();
                self.step();
                break;
            }

            self.step();
        }
    }

    fn version(&mut self, start: usize) -> Result<(usize, String)> {
        let (_, _) = take!(self, start, ' ' | '\n' | '\r' | '\t');

        let (end, buffer) =
            take!(self, start, '^' | '<' | '>' | '=' | '.' | '-' | '0'...'9' | 'a'...'z');

        Ok((end, buffer.to_owned()))
    }

    fn illegal<T>(&mut self) -> Result<T> {
        self.illegal = true;
        Err(ErrorKind::IllegalToken(self.pos()).into())
    }
}

impl<I> Iterator for Lexer<I>
    where I: Iterator<Item = (usize, char)>
{
    type Item = Result<(usize, Token, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.illegal {
            return Some(self.illegal());
        }

        // code block mode
        if let Some(start) = self.code_block {
            return Some(self.code_block(start));
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
                    ('}', '}') => Some(Token::CodeClose),
                    ('{', '{') => {
                        self.code_block = Some(start + 2);
                        Some(Token::CodeOpen)
                    }
                    (':', ':') => Some(Token::Scope),
                    ('=', '>') => Some(Token::HashRocket),
                    _ => None,
                };

                if let Some(token) = token {
                    self.step();
                    self.step();
                    self.last_comment.clear();
                    return Some(Ok((start, token, start + 2)));
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
                        self.step();

                        return Some(self.version(start)
                            .map(|(end, version)| (start, Token::Version(version), end)));
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

pub fn lex<I>(input: I) -> Lexer<I>
    where I: Iterator<Item = (usize, char)>
{
    Lexer {
        source: input,
        n0: None,
        n1: None,
        last_comment: Vec::new(),
        buffer: String::new(),
        illegal: false,
        code_block: None,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::Token::*;

    fn tokenize(input: &str) -> Result<Vec<(usize, Token, usize)>> {
        lex(input.char_indices()).collect()
    }

    #[test]
    pub fn test_lexer() {
        let expected = vec![(0, Identifier(Commented::empty("hello".into())), 5),
                            (6, TypeIdentifier(Commented::empty("World".into())), 11),
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
                            (2, CodeContent(" foo bar baz \n zing ".into()), 22),
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
                             8)];

        assert_eq!(expected, tokenize("-12.42e-4").unwrap());
    }

    #[test]
    pub fn test_number_2() {
        assert_eq!(vec![(0, Number(12.into()), 1)], tokenize("12").unwrap());
    }

    #[test]
    pub fn test_name() {
        let expected = vec![(0, Identifier(Commented::empty("foo".into())), 3),
                            (3, Scope, 5),
                            (5, TypeIdentifier(Commented::empty("Bar".into())), 8),
                            (8, Dot, 9),
                            (9, TypeIdentifier(Commented::empty("Baz".into())), 12)];

        assert_eq!(expected, tokenize("foo::Bar.Baz").unwrap());
    }

    #[test]
    pub fn test_instance() {
        let expected = vec![(0, Identifier(Commented::empty("foo".into())), 3),
                            (3, Scope, 5),
                            (5, TypeIdentifier(Commented::empty("Bar".into())), 8),
                            (8, Dot, 9),
                            (9, TypeIdentifier(Commented::empty("Baz".into())), 12),
                            (12, LeftParen, 13),
                            (13, Identifier(Commented::empty("hello".into())), 18),
                            (18, Colon, 19),
                            (20, Number(12.into()), 22),
                            (22, RightParen, 23)];

        assert_eq!(expected, tokenize("foo::Bar.Baz(hello: 12)").unwrap());
    }

    #[test]
    pub fn test_comments() {
        let tokens = tokenize("// hello \n world");
        let comment = vec![" hello ".into()];
        assert_eq!(vec![(11, Identifier(Commented::new(comment, "world".into())), 15)],
                   tokens.unwrap());

        let tokens = tokenize("he/* this is a comment */llo");
        assert_eq!(vec![(0, Identifier(Commented::empty("hello".into())), 27)],
                   tokens.unwrap());

        let tokens = tokenize("// test\n// this\nhello");
        let comment = vec![" test".into(), " this".into()];

        assert_eq!(vec![(16, Identifier(Commented::new(comment, "hello".into())), 20)],
                   tokens.unwrap());
    }

    #[test]
    pub fn test_version_req() {
        let tokens = tokenize("@[>=1.0]");
        println!("tokens = {:?}", tokens);
    }
}
