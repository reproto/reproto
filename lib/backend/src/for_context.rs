use core::{Loc, RpCode};

pub trait ForContext {
    type Item;

    fn for_context(self, context: &str) -> Vec<Self::Item>;
}

// TODO: borrow content
impl<'a, T> ForContext for T
where
    T: IntoIterator<Item = &'a Loc<RpCode>>,
    Self: 'a,
{
    type Item = <Self as IntoIterator>::Item;

    fn for_context(self, context: &str) -> Vec<Self::Item> {
        self.into_iter()
            .filter(|c| Loc::as_ref(c).context == context)
            .collect()
    }
}
