use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceAccepts {
    pub comment: Vec<String>,
    pub ty: RpLoc<RpType>,
    pub accepts: Option<Mime>,
}
