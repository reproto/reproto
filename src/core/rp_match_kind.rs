/// Simplified types that _can_ be uniquely matched over for JSON.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RpMatchKind {
    Any,
    Object,
    Array,
    String,
    Boolean,
    Number,
}
