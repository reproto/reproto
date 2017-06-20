pub use super::*;

pub trait ForContext {
    type Output;

    fn for_context(&self, context: &str) -> Self::Output;
}

// TODO: borrow content
impl ForContext for Vec<RpLoc<RpCode>> {
    type Output = Vec<RpLoc<RpCode>>;

    fn for_context(&self, context: &str) -> Self::Output {
        self.iter().filter(|c| c.as_ref().context == context).map(|c| c.clone()).collect()
    }
}
