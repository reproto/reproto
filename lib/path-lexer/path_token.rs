#[derive(Clone, Debug, PartialEq)]
pub enum PathToken<'input> {
    /// Variable identifier.
    Identifier(&'input str),
    /// Potentially escaped segment.
    Segment(String),
    Slash,
    LeftCurly,
    RightCurly,
}
