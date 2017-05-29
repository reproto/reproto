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

    pub fn map_inner<M, U>(&self, map: M) -> Token<U, P>
        where M: FnOnce(&T) -> U,
              U: Clone
    {
        Token::new(map(&self.inner), self.pos.clone())
    }
}
