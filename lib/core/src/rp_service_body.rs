//! Model for services.

use super::{Loc, RpEndpoint};

#[derive(Debug, Clone, Serialize, Default)]
pub struct RpServiceBodyHttp {
    /// Default URL to use for service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Loc<String>>,
}

decl_body!(pub struct RpServiceBody {
    pub http: RpServiceBodyHttp,
    pub endpoints: Vec<Loc<RpEndpoint>>,
});
