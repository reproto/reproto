use reproto_core::RpNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commented<T> {
    pub comment: Vec<String>,
    pub value: T,
}

impl<T> Commented<T> {
    pub fn new(comment: Vec<String>, value: T) -> Commented<T> {
        Commented {
            comment: comment,
            value: value,
        }
    }

    pub fn empty(value: T) -> Commented<T> {
        Commented {
            comment: vec![],
            value: value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(Commented<String>),
    TypeIdentifier(Commented<String>),
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
    CodeContent(String),
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
    EndpointKeyword(Vec<String>),
    ReturnsKeyword(Vec<String>),
    Star(Vec<String>),
}
