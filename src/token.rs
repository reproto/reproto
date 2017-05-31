use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<T, P> {
    pub inner: T,
    pub pos: P,
}

impl<T, P> ::std::ops::Deref for Token<T, P> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, P> Token<T, P>
    where P: Clone
{
    pub fn new(inner: T, pos: P) -> Token<T, P> {
        Token {
            inner: inner,
            pos: pos,
        }
    }

    pub fn map_inner<M, U>(self, map: M) -> Token<U, P>
        where M: FnOnce(T) -> U
    {
        Token::new(map(self.inner), self.pos)
    }
}

pub trait WithPrefix<T> {
    type Prefix;
    type Output;

    fn with_prefix(self, prefix: Self::Prefix) -> Token<T, Self::Output>;
}

impl<T, B, C> WithPrefix<T> for Token<T, (B, C)>
    where T: Clone,
          B: Clone,
          C: Clone
{
    type Prefix = PathBuf;
    type Output = (PathBuf, B, C);

    fn with_prefix(self, prefix: Self::Prefix) -> Token<T, Self::Output> {
        let (b, c) = self.pos.clone();
        Token::new(self.inner.clone(), (prefix, b, c))
    }
}
