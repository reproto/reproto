use super::rp_path_fragment::RpPathFragment;

#[derive(Debug, PartialEq, Clone)]
pub struct RpPath {
    pub fragments: Vec<RpPathFragment>,
}
