#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpModifier {
    Required,
    Optional,
    Repeated,
}
