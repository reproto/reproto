/// Simplified types that _can_ be uniquely matched over for JSON.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpMatchKind {
    Any,
    Object,
    Array,
    String,
    Boolean,
    Number,
}
