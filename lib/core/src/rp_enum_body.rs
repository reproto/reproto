//! Model for enums

use {Loc, RpCode, RpEnumType, RpVariant};
use std::rc::Rc;

decl_body!(pub struct RpEnumBody {
    /// The type of the variant.
    pub enum_type: RpEnumType,
    /// Variants in the enum.
    pub variants: Vec<Rc<Loc<RpVariant>>>,
    /// Custom code blocks in the enum.
    pub codes: Vec<Loc<RpCode>>,
});
