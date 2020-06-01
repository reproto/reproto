//! Lexer for paths.

use crate::errors::Result;
use crate::regex_token::RegexToken;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Normal,
    CharacterClass,
}

pub struct RegexLexer<'input> {
    source: CharIndices<'input>,
    source_len: usize,
    #[allow(dead_code)]
    source_str: &'input str,
    n0: Option<(usize, char)>,
    n1: Option<(usize, char)>,
    /// Escape the next character.
    /// Stored offset is the start of the escape sequence.
    escape: Option<usize>,
    /// When we encounter a bracket, switch to group mode which has special rules for certain
    /// symbols.
    mode: Mode,
}

impl<'input> RegexLexer<'input> {
    /// Advance the source iterator.
    #[inline]
    fn step(&mut self) {
        self.n0 = self.n1;
        self.n1 = self.source.next();
    }

    #[inline]
    fn step_n(&mut self, n: usize) -> Option<usize> {
        for _ in 0..n {
            self.step();
        }

        self.n0.map(|v| v.0)
    }

    #[inline]
    fn one(&mut self) -> Option<(usize, char)> {
        self.n0
    }

    fn token(&mut self, start: usize, token: RegexToken) -> Result<(usize, RegexToken, usize)> {
        let end = self.step_n(1).unwrap_or(self.source_len);
        Ok((start, token, end))
    }

    fn normal_mode_next(&mut self) -> Option<Result<(usize, RegexToken, usize)>> {
        use self::RegexToken::*;

        // one character keywords
        while let Some((pos, c)) = self.one() {
            if let Some(pos) = self.escape.take() {
                return Some(self.token(pos, Character(c)));
            }

            let out = match c {
                '\\' => {
                    self.escape = Some(pos);
                    self.step();
                    continue;
                }
                '[' => {
                    self.mode = Mode::CharacterClass;
                    self.token(pos, LeftBracket)
                }
                '+' => self.token(pos, Plus),
                '*' => self.token(pos, Star),
                '?' => self.token(pos, QuestionMark),
                '.' => self.token(pos, Dot),
                '^' => self.token(pos, Bracket),
                '$' => self.token(pos, Dollar),
                c => self.token(pos, Character(c)),
            };

            return Some(out);
        }

        None
    }

    /// Special mode: consume characters until we hit the next matching bracket.
    fn character_class_mode_next(&mut self) -> Option<Result<(usize, RegexToken, usize)>> {
        use self::RegexToken::*;

        while let Some((pos, c)) = self.one() {
            if let Some(pos) = self.escape.take() {
                return Some(self.token(pos, Character(c)));
            }

            let out = match c {
                '\\' => {
                    self.escape = Some(pos);
                    continue;
                }
                ']' => {
                    self.mode = Mode::Normal;
                    self.token(pos, RightBracket)
                }
                '-' => self.token(pos, Dash),
                c => self.token(pos, Character(c)),
            };

            return Some(out);
        }

        None
    }
}

impl<'input> Iterator for RegexLexer<'input> {
    type Item = Result<(usize, RegexToken, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            Mode::Normal => self.normal_mode_next(),
            Mode::CharacterClass => self.character_class_mode_next(),
        }
    }
}

pub fn regex_lex(input: &str) -> RegexLexer {
    let mut source = input.char_indices();

    let n0 = source.next();
    let n1 = source.next();

    RegexLexer {
        source: source,
        source_len: input.len(),
        source_str: input,
        n0: n0,
        n1: n1,
        escape: None,
        mode: Mode::Normal,
    }
}

#[cfg(test)]
pub mod tests {
    use super::RegexToken::*;
    use super::*;

    fn tokenize(input: &str) -> Result<Vec<(usize, RegexToken, usize)>> {
        regex_lex(input).collect()
    }

    #[test]
    pub fn test_regex_lexer() {
        let input = "[a-z+*?.]+*?.\\+\\*\\?\\.";

        let expected = vec![
            (0, LeftBracket, 1),
            (1, Character('a'), 2),
            (2, Dash, 3),
            (3, Character('z'), 4),
            (4, Character('+'), 5),
            (5, Character('*'), 6),
            (6, Character('?'), 7),
            (7, Character('.'), 8),
            (8, RightBracket, 9),
            (9, Plus, 10),
            (10, Star, 11),
            (11, QuestionMark, 12),
            (12, Dot, 13),
            (13, Character('+'), 15),
            (15, Character('*'), 17),
            (17, Character('?'), 19),
            (19, Character('.'), 21),
        ];

        assert_eq!(expected, tokenize(input).unwrap());
    }
}
