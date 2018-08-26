#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegexToken {
    LeftBracket,
    RightBracket,
    Dash,
    Star,
    Plus,
    Dot,
    QuestionMark,
    Bracket,
    Dollar,
    Character(char),
}
