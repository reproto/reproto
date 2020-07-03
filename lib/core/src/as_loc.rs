//! Utility trait to convert various wrapper types into locations.

use super::Spanned;
use std::rc::Rc;

pub trait AsLoc {
    type Output;

    fn as_loc(self) -> Spanned<Self::Output>;
}

impl<T> AsLoc for Spanned<T> {
    type Output = T;

    fn as_loc(self) -> Spanned<Self::Output> {
        self
    }
}

impl<'a, T> AsLoc for &'a Spanned<T> {
    type Output = &'a T;

    fn as_loc(self) -> Spanned<Self::Output> {
        Spanned::as_ref(self)
    }
}

impl<'a, T> AsLoc for &'a Rc<Spanned<T>> {
    type Output = &'a T;

    fn as_loc(self) -> Spanned<Self::Output> {
        Spanned::as_ref(self)
    }
}
