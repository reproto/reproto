//! Model for enums

use super::{Loc, RpCode, RpEnumType, RpVariant};
use std::rc::Rc;

decl_body!(pub struct RpEnumBody {
    /// The type of the variant.
    pub variant_type: RpEnumType,
    pub variants: Vec<Rc<Loc<RpVariant>>>,
    pub codes: Vec<Loc<RpCode>>,
});
