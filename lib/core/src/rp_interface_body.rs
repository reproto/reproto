//! Model for tuples.

use super::{Loc, RpCode, RpField, RpSubType};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::slice;

decl_body!(pub struct RpInterfaceBody {
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    pub sub_types: BTreeMap<String, Rc<Loc<RpSubType>>>,
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

impl RpInterfaceBody {
    pub fn fields(&self) -> Fields {
        Fields {
            iter: self.fields.iter(),
        }
    }
}
