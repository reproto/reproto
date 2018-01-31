//! Utility trait to convert various wrapper types into locations.

use super::Loc;
use std::rc::Rc;

pub trait AsLoc {
    type Output;

    fn as_loc(self) -> Loc<Self::Output>;
}

impl<T> AsLoc for Loc<T> {
    type Output = T;

    fn as_loc(self) -> Loc<Self::Output> {
        self
    }
}

impl<'a, T: 'a> AsLoc for &'a Loc<T> {
    type Output = &'a T;

    fn as_loc(self) -> Loc<Self::Output> {
        Loc::as_ref(self)
    }
}

impl<'a, T: 'a> AsLoc for &'a Rc<Loc<T>> {
    type Output = &'a T;

    fn as_loc(self) -> Loc<Self::Output> {
        Loc::as_ref(self)
    }
}
