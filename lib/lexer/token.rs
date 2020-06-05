use core::RpNumber;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    // identifier-style keywords
    Any,
    As,
    Boolean,
    Bytes,
    Datetime,
    Enum,
    Float,
    Double,
    I32,
    I64,
    Interface,
    Service,
    Stream,
    String,
    Tuple,
    Type,
    U32,
    U64,
    Use,
}

impl Keyword {
    /// Get the keywords-safe variant of the given keyword.
    pub fn keyword_safe(&self) -> &'static str {
        match *self {
            Self::Any => "_any",
            Self::As => "_as",
            Self::Boolean => "_boolean",
            Self::Bytes => "_bytes",
            Self::Datetime => "_datetime",
            Self::Enum => "_enum",
            Self::Float => "_float",
            Self::Double => "_double",
            Self::I32 => "_i32",
            Self::I64 => "_i64",
            Self::Interface => "_interface",
            Self::Service => "_service",
            Self::Stream => "_stream",
            Self::String => "_string",
            Self::Tuple => "_tuple",
            Self::Type => "_type",
            Self::U32 => "_u32",
            Self::U64 => "_u64",
            Self::Use => "_use",
        }
    }

    /// Treat keyword as an identifier.
    pub fn as_ident(&self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Interface => "interface",
            Self::Type => "type",
            Self::Enum => "enum",
            Self::Tuple => "tuple",
            Self::Service => "service",
            Self::Use => "use",
            Self::As => "as",
            Self::Float => "float",
            Self::Double => "double",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Boolean => "boolean",
            Self::String => "string",
            Self::Datetime => "datetime",
            Self::Bytes => "bytes",
            Self::Stream => "stream",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'input> {
    Identifier(Cow<'input, str>),
    TypeIdentifier(Cow<'input, str>),
    PackageDocComment(Vec<Cow<'input, str>>),
    DocComment(Vec<Cow<'input, str>>),
    Number(RpNumber),
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    SemiColon,
    Colon,
    Equal,
    Comma,
    Dot,
    Scope,
    QuestionMark,
    Hash,
    Bang,
    RightArrow,
    CodeOpen,
    CodeClose,
    CodeContent(Cow<'input, str>),
    QuotedString(String),
    Keyword(Keyword),
}

impl<'input> Token<'input> {
    pub fn as_ident(&self) -> Option<&str> {
        let ident = match *self {
            Self::Keyword(kw) => kw.as_ident(),
            Self::Identifier(ref ident) => ident.as_ref(),
            _ => return None,
        };

        Some(ident)
    }
}
