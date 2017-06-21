use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceEndpoint {
    pub url: String,
    pub comment: Vec<String>,
    pub accepts: Vec<RpServiceAccepts>,
    pub returns: Vec<RpServiceReturns>,
    pub method: Option<String>,
}
