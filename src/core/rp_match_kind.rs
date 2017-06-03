/// Simplified types that _can_ be uniquely matched over for JSON.
#[derive(Debug, PartialEq, Clone)]
pub enum RpMatchKind {
    Any,
    Object,
    Array,
    String,
    Boolean,
    Number,
}
