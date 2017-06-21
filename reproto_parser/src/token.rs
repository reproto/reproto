use reproto_core::RpNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commented<'input, T> {
    pub comment: Vec<&'input str>,
    pub value: T,
}

pub fn commented<'input, T>(comment: Vec<&'input str>, value: T) -> Commented<'input, T> {
    Commented {
        comment: comment,
        value: value,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Identifier(Commented<'input, &'input str>),
    TypeIdentifier(Commented<'input, &'input str>),
    Number(RpNumber),
    Version(String),
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    SemiColon,
    Colon,
    Comma,
    Dot,
    Scope,
    Optional,
    And,
    Slash,
    Equals,
    HashRocket,
    CodeOpen,
    CodeClose,
    CodeContent(&'input str),
    String(String),
    // identifier-style keywords
    InterfaceKeyword,
    TypeKeyword,
    EnumKeyword,
    TupleKeyword,
    ServiceKeyword,
    PackageKeyword,
    MatchKeyword,
    UseKeyword,
    AsKeyword,
    AnyKeyword,
    OnKeyword,
    FloatKeyword,
    DoubleKeyword,
    SignedKeyword,
    UnsignedKeyword,
    BooleanKeyword,
    StringKeyword,
    BytesKeyword,
    TrueKeyword,
    FalseKeyword,
    EndpointKeyword(Vec<&'input str>),
    ReturnsKeyword(Vec<&'input str>),
    AcceptsKeyword(Vec<&'input str>),
    Star(Vec<&'input str>),
}
