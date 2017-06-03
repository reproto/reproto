use loc;

pub trait WithPrefix<U> {
    type Output;

    fn with_prefix(self, prefix: U) -> Self::Output;
}

impl<T, A, B, U> WithPrefix<U> for loc::Loc<T, (A, B)>
    where U: Clone,
          A: Clone,
          B: Clone
{
    type Output = loc::Loc<T, (U, A, B)>;

    fn with_prefix(self, prefix: U) -> loc::Loc<T, (U, A, B)> {
        loc::Loc::new(self.inner, (prefix, self.pos.0, self.pos.1))
    }
}
