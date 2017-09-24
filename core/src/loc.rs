use super::errors::*;
use super::merge::Merge;
use pos::Pos;
use serde;
use std::cmp;
use std::hash;
use std::result;

#[derive(Clone)]
pub struct Loc<T> {
    inner: T,
    pos: Pos,
}

impl<T: serde::Serialize> serde::Serialize for Loc<T> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<T> Loc<T> {
    pub fn new<P: Into<Pos>>(inner: T, pos: P) -> Loc<T> {
        Loc {
            inner: inner,
            pos: pos.into(),
        }
    }

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    pub fn take(self) -> T {
        self.inner
    }

    pub fn take_pair(self) -> (T, Pos) {
        (self.inner, self.pos)
    }

    pub fn as_ref_pair(&self) -> (&T, &Pos) {
        (&self.inner, &self.pos)
    }

    pub fn map<'a, M, U>(&'a self, map: M) -> Loc<U>
    where
        M: FnOnce(&'a T) -> U,
        U: 'a,
    {
        Loc::new(map(&self.inner), self.pos.clone())
    }
}

impl<T> cmp::PartialEq for Loc<T>
where
    T: cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(other)
    }
}

impl<T> cmp::Eq for Loc<T>
where
    T: cmp::Eq,
{
}

impl<T> cmp::PartialOrd for Loc<T>
where
    T: cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(other)
    }
}

impl<T> cmp::Ord for Loc<T>
where
    T: cmp::Ord,
    Self: cmp::PartialOrd + cmp::Eq,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(other)
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

impl<T> Merge for Loc<T>
where
    T: Merge,
{
    fn merge(&mut self, source: Loc<T>) -> Result<()> {
        self.as_mut().merge(source.take())?;
        Ok(())
    }
}

impl<T> ::std::ops::Deref for Loc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> ::std::borrow::Borrow<T> for Loc<T> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T> ::std::convert::AsRef<T> for Loc<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> ::std::convert::AsMut<T> for Loc<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> ::std::fmt::Display for Loc<T>
where
    T: ::std::fmt::Display,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T> ::std::fmt::Debug for Loc<T>
where
    T: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "<{:?}@{:?}>", self.inner, self.pos)
    }
}
