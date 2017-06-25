use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpServiceBody {
    pub name: String,
    pub comment: Vec<String>,
    pub endpoints: Vec<RpServiceEndpoint>,
}

impl Merge for RpServiceBody {
    fn merge(&mut self, _source: RpServiceBody) -> Result<()> {
        Ok(())
    }
}
