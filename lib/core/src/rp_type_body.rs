//! Model for tuples.

use super::{Loc, RpCode, RpField};
use std::collections::HashSet;
use std::slice;

decl_body!(pub struct RpTypeBody {
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    // Set of fields which are reserved for this type.
    pub reserved: HashSet<Loc<String>>,
});

/// Iterator over fields.
pub struct Fields<'a> {
    iter: slice::Iter<'a, Loc<RpField>>,
}

impl<'a> Iterator for Fields<'a> {
    type Item = &'a Loc<RpField>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl RpTypeBody {
    pub fn fields(&self) -> Fields {
        Fields {
            iter: self.fields.iter(),
        }
    }
}
