use super::rp_type::RpType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RpPathFragment {
    Variable { name: String, ty: RpType },
}
