use super::rp_type::RpType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpPathFragment {
    Variable { name: String, ty: RpType },
}
