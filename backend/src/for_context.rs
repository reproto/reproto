use core::{Loc, RpCode};

pub trait ForContext {
    type Output;

    fn for_context(&self, context: &str) -> Self::Output;
}

// TODO: borrow content
impl ForContext for Vec<Loc<RpCode>> {
    type Output = Vec<Loc<RpCode>>;

    fn for_context(&self, context: &str) -> Self::Output {
        self.iter()
            .filter(|c| c.as_ref().context == context)
            .map(|c| c.clone())
            .collect()
    }
}
