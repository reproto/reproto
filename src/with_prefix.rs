use token;

pub trait WithPrefix<U> {
    type Output;

    fn with_prefix(self, prefix: U) -> Self::Output;
}

impl<T, A, B, U> WithPrefix<U> for token::Token<T, (A, B)>
    where U: Clone,
          A: Clone,
          B: Clone
{
    type Output = token::Token<T, (U, A, B)>;

    fn with_prefix(self, prefix: U) -> token::Token<T, (U, A, B)> {
        token::Token::new(self.inner, (prefix, self.pos.0, self.pos.1))
    }
}
