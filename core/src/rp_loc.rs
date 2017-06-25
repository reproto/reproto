use loc;
use std::path::PathBuf;
use std::rc::Rc;
use super::errors::*;
use super::merge::Merge;

pub type RpPos = (Rc<PathBuf>, usize, usize);
pub type RpLoc<T> = loc::Loc<T, RpPos>;

impl<T> Merge for RpLoc<T>
    where T: Merge
{
    fn merge(&mut self, source: RpLoc<T>) -> Result<()> {
        self.as_mut().merge(source.move_inner())?;
        Ok(())
    }
}
