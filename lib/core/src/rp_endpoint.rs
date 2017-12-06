//! Model for endpoints

use super::{Attributes, Loc, RpChannel, RpPathSpec};
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone, Serialize)]
pub enum RpHttpMethod {
    GET,
    POST,
    PUT,
    UPDATE,
    DELETE,
    PATCH,
    HEAD,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RpEndpointHttp {
    /// Path specification.
    pub path: Option<RpPathSpec>,
    /// Argument that is the body of the request.
    pub body: Option<String>,
    /// HTTP method.
    pub method: Option<RpHttpMethod>,
}

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
    pub arguments: LinkedHashMap<String, (Loc<String>, Loc<RpChannel>)>,
    /// Response type that this endpoint responds with.
    pub response: Option<Loc<RpChannel>>,
    /// HTTP configuration.
    pub http: RpEndpointHttp,
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
