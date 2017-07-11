use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Loc<T, P> {
    #[serde(rename = "value")]
    inner: T,
    pos: P,
}

impl<T, P> ::std::ops::Deref for Loc<T, P> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, P> ::std::borrow::Borrow<T> for Loc<T, P> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T, P> ::std::convert::AsRef<T> for Loc<T, P> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T, P> ::std::convert::AsMut<T> for Loc<T, P> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T, P> Loc<T, P>
    where P: Clone
{
    pub fn new(inner: T, pos: P) -> Loc<T, P> {
        Loc {
            inner: inner,
            pos: pos,
        }
    }

    pub fn move_inner(self) -> T {
        self.inner
    }

    pub fn pos(&self) -> &P {
        &self.pos
    }

    pub fn map_into<M, U>(self, map: M) -> Loc<U, P>
        where M: FnOnce(T) -> U
    {
        Loc::new(map(self.inner), self.pos)
    }

    pub fn map<'a, M, U>(&'a self, map: M) -> Loc<U, P>
        where M: FnOnce(&'a T) -> U,
              U: 'a
    {
        Loc::new(map(&self.inner), self.pos.clone())
    }

    pub fn both(self) -> (T, P) {
        (self.inner, self.pos)
    }

    pub fn ref_both(&self) -> (&T, &P) {
        (&self.inner, &self.pos)
    }
}

impl<T, P> ::std::fmt::Display for Loc<T, P>
    where T: ::std::fmt::Display
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T, P> ::std::fmt::Debug for Loc<T, P>
    where T: ::std::fmt::Debug,
          P: ::std::fmt::Debug
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "<{:?}@{:?}>", self.inner, self.pos)
    }
}
