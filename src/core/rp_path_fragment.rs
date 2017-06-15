use super::rp_type::RpType;

#[derive(Debug, PartialEq, Clone)]
pub enum RpPathFragment {
    Variable { name: String, ty: RpType },
}
