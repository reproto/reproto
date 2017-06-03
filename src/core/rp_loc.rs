use loc;
use parser::ast;
use std::path::PathBuf;
use super::errors::*;
use super::into_model::IntoModel;
use super::merge::Merge;

pub type RpPos = (PathBuf, usize, usize);
pub type RpLoc<T> = loc::Loc<T, RpPos>;

impl<T> Merge for RpLoc<T>
    where T: Merge
{
    fn merge(&mut self, source: RpLoc<T>) -> Result<()> {
        self.inner.merge(source.inner)?;
        Ok(())
    }
}

impl<T> IntoModel for ast::AstLoc<T>
    where T: IntoModel
{
    type Output = RpLoc<T::Output>;

    fn into_model(self, pos: &RpPos) -> Result<Self::Output> {
        let pos = (pos.0.clone(), self.pos.0, self.pos.1);
        let out = self.inner.into_model(&pos)?;
        Ok(RpLoc::new(out, pos))
    }
}
