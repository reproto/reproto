use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum PathToken<'input> {
    /// Variable identifier.
    Identifier(Cow<'input, str>),
    /// Potentially escaped segment.
    Segment(String),
    Slash,
    LeftCurly,
    RightCurly,
}
