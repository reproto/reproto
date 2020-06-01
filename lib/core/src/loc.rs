use crate::Span;
use serde;
use std::borrow;
use std::cmp;
use std::fmt;
use std::hash;
use std::ops;
use std::result;

/// Loc is a value and a span combined.
///
/// The span indicates a byte range where the value was extracted from.
#[derive(Clone)]
pub struct Loc<T> {
    inner: T,
    span: Span,
}

impl<T> Loc<T> {
    /// Create a new spanned item.
    pub fn new<P: Into<Span>>(inner: T, span: P) -> Loc<T> {
        Loc {
            inner,
            span: span.into(),
        }
    }

    /// Access the span of the item.
    pub fn span(loc: &Loc<T>) -> Span {
        loc.span
    }

    /// Consume the loc and take the value of it.
    pub fn take(loc: Loc<T>) -> T {
        loc.inner
    }

    /// Consume the loc and take the value and the span.
    pub fn take_pair(loc: Loc<T>) -> (T, Span) {
        (loc.inner, loc.span)
    }

    /// Borrow the value from a loc.
    pub fn borrow(loc: &Loc<T>) -> &T {
        &loc.inner
    }

    /// Borrow a value and the span from a loc.
    pub fn borrow_pair(loc: &Loc<T>) -> (&T, Span) {
        (&loc.inner, loc.span)
    }

    /// Map the loc.
    pub fn map<U, O>(loc: Loc<T>, op: O) -> Loc<U>
    where
        O: FnOnce(T) -> U,
    {
        Loc::new(op(loc.inner), loc.span)
    }

    /// Convert a reference to a loc, into a loc of a reference.
    pub fn as_ref(loc: &Loc<T>) -> Loc<&T> {
        Loc::new(&loc.inner, loc.span)
    }

    /// Apply the fallible operation over the given location.
    pub fn and_then<U, O, E>(Loc { inner, span }: Loc<T>, op: O) -> result::Result<Loc<U>, E>
    where
        O: FnOnce(T) -> result::Result<U, E>,
    {
        op(inner).map(|value| Loc::new(value, span))
    }
}

impl<T: serde::Serialize> serde::Serialize for Loc<T> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<T> cmp::PartialEq for Loc<T>
where
    T: cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> cmp::Eq for Loc<T> where T: cmp::Eq {}

impl<T> cmp::PartialOrd for Loc<T>
where
    T: cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> cmp::Ord for Loc<T>
where
    T: cmp::Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> hash::Hash for Loc<T>
where
    T: hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.inner.hash(state)
    }
}

impl<T> ops::Deref for Loc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> ops::DerefMut for Loc<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> ::std::convert::AsMut<T> for Loc<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> borrow::Borrow<T> for Loc<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T> fmt::Display for Loc<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T> fmt::Debug for Loc<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{:?}@{:?}>", self.inner, self.span)
    }
}

impl<'a, T> From<&'a Loc<T>> for Span {
    fn from(value: &'a Loc<T>) -> Self {
        Loc::span(value)
    }
}
