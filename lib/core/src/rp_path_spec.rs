//! Path specifications

/// A part of a step.
#[derive(Debug, PartialEq, Eq)]
pub enum RpPathPart {
    Variable(String),
    Segment(String),
}

/// A step in a path specification.
#[derive(Debug, PartialEq, Eq)]
pub struct RpPathStep {
    pub parts: Vec<RpPathPart>,
}

/// A path specification.
#[derive(Debug, PartialEq, Eq)]
pub struct RpPathSpec {
    pub segments: Vec<RpPathStep>,
}
