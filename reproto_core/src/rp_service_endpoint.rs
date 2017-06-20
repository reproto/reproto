use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceEndpoint {
    pub name: String,
    pub comment: Vec<String>,
    pub responses: Vec<RpServiceResponse>,
}
