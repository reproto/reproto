#[derive(Debug, Clone, Serialize)]
pub struct RpServiceResponse {
    pub name: String,
    pub comment: Vec<String>,
}
