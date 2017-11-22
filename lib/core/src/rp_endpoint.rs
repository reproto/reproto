//! Model for endpoints

use super::{Attributes, Loc, RpChannel};

#[derive(Debug, Clone, Serialize)]
pub struct RpEndpoint {
    /// Name of the endpoint. Guaranteed to be unique.
    pub id: Loc<String>,
    /// Name of the endpoint. This is the name which is being sent over the wire.
    pub name: String,
    /// Comments for documentation.
    pub comment: Vec<String>,
    /// Attributes associated with the endpoint.
    pub attributes: Attributes,
    /// Request type that this endpoint expects.
    pub request: Option<Loc<RpChannel>>,
    /// Response type that this endpoint responds with.
    pub response: Option<Loc<RpChannel>>,
}

impl RpEndpoint {
    pub fn id_parts<F>(&self, filter: F) -> Vec<String>
    where
        F: Fn(&str) -> String,
    {
        vec![filter(self.id.as_str())]
    }

    /// Get the name of the endpoint.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
