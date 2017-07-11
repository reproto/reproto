use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceAccepts {
    pub comment: Vec<String>,
    pub ty: Loc<RpType>,
    pub accepts: Option<Mime>,
}
