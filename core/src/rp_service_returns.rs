use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceReturns {
    pub comment: Vec<String>,
    pub ty: Option<RpLoc<RpType>>,
    pub produces: Option<Mime>,
    pub status: Option<u32>,
}
