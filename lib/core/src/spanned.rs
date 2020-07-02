use crate::Span;
use serde;
use serde::Serialize;
use std::borrow;
use std::cmp;
use std::fmt;
use std::hash;
use std::ops;
use std::result;

/// Spanned is a value and a span combined.
///
/// The span indicates a byte range where the value was extracted from.
#[derive(Clone)]
pub struct Spanned<T> {
    inner: T,
    span: Span,
}

impl<T> Spanned<T> {
    /// Create a new spanned item.
    pub fn new<P: Into<Span>>(inner: T, span: P) -> Spanned<T> {
        Spanned {
            inner,
            span: span.into(),
        }
    }

    /// Access the span of the item.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Consume the spanned and take the value of it.
    pub fn take(spanned: Spanned<T>) -> T {
        spanned.inner
    }

    /// Consume the spanned and take the value and the span.
    pub fn take_pair(spanned: Spanned<T>) -> (T, Span) {
        (spanned.inner, spanned.span)
    }

    /// Borrow the value from a spanned.
    pub fn borrow(spanned: &Spanned<T>) -> &T {
        &spanned.inner
    }

    /// Borrow a value and the span from a spanned.
    pub fn borrow_pair(spanned: &Spanned<T>) -> (&T, Span) {
        (&spanned.inner, spanned.span)
    }

    /// Map the spanned.
    pub fn map<U, O>(spanned: Spanned<T>, op: O) -> Spanned<U>
    where
        O: FnOnce(T) -> U,
    {
        Spanned::new(op(spanned.inner), spanned.span)
    }

    /// Convert a reference to a spanned, into a reference.
    pub fn as_ref(spanned: &Spanned<T>) -> Spanned<&T> {
        Spanned::new(&spanned.inner, spanned.span)
    }

    /// Apply the fallible operation over the given location.
    pub fn and_then<U, O, E>(
        Spanned { inner, span }: Spanned<T>,
        op: O,
    ) -> result::Result<Spanned<U>, E>
    where
        O: FnOnce(T) -> result::Result<U, E>,
    {
        op(inner).map(|value| Spanned::new(value, span))
    }
}

impl<T: Serialize> Serialize for Spanned<T> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<T> cmp::PartialEq for Spanned<T>
where
    T: cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> cmp::Eq for Spanned<T> where T: cmp::Eq {}

impl<T> cmp::PartialOrd for Spanned<T>
where
    T: cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> cmp::Ord for Spanned<T>
where
    T: cmp::Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> hash::Hash for Spanned<T>
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

impl<T> ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> ops::DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> ::std::convert::AsMut<T> for Spanned<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> borrow::Borrow<T> for Spanned<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T> fmt::Display for Spanned<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T> fmt::Debug for Spanned<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{:?}@{:?}>", self.inner, self.span)
    }
}

impl<'a, T> From<&'a Spanned<T>> for Span {
    fn from(value: &'a Spanned<T>) -> Self {
        value.span()
    }
}
