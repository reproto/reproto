//! Model for sub-types

use super::{Loc, RpCode, RpDecl, RpField, RpName};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize)]
pub struct RpSubType {
    pub name: RpName,
    pub local_name: String,
    pub comment: Vec<String>,
    /// Inner declarations.
    pub decls: Vec<Rc<Loc<RpDecl>>>,
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    pub sub_type_name: Option<Loc<String>>,
}

impl RpSubType {
    pub fn name(&self) -> &str {
        self.sub_type_name.as_ref().map(|t| t.as_str()).unwrap_or(
            &self.local_name,
        )
    }
}
