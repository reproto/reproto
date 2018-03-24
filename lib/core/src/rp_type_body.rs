//! Model for tuples.

use {Flavor, Loc, RpCode, RpField};
use std::slice;

decl_body!(pub struct RpTypeBody<F> {
    pub fields: Vec<Loc<RpField<F>>>,
    pub codes: Vec<Loc<RpCode>>,
});

/// Iterator over fields.
pub struct Fields<'a, F: 'static>
where
    F: Flavor,
{
    iter: slice::Iter<'a, Loc<RpField<F>>>,
}

impl<'a, F: 'static> Iterator for Fields<'a, F>
where
    F: Flavor,
{
    type Item = &'a Loc<RpField<F>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<F> RpTypeBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> Fields<F> {
        Fields {
            iter: self.fields.iter(),
        }
    }
}
