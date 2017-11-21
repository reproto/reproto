//! Model for services.

use super::{Loc, RpEndpoint};
use linked_hash_map::LinkedHashMap;

decl_body!(pub struct RpServiceBody {
    pub endpoints: LinkedHashMap<String, Loc<RpEndpoint>>,
});
