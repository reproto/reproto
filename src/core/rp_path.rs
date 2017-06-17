use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct RpPath {
    pub fragments: Vec<RpPathFragment>,
}
