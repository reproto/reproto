use super::loc::Loc;
use super::with_pos::WithPos;
use std::result;

/// Helper trait to iterate over a collection of loc items.
pub trait ForEachLoc {
    type Item;

    fn for_each_loc<F, E: WithPos>(self, callback: F) -> result::Result<(), E>
    where
        F: FnMut(Self::Item) -> result::Result<(), E>;
}

impl<'a, T: 'a, I> ForEachLoc for I
where
    I: IntoIterator<Item = &'a Loc<T>>,
{
    type Item = &'a T;

    fn for_each_loc<F, E: WithPos>(self, mut callback: F) -> result::Result<(), E>
    where
        F: FnMut(Self::Item) -> result::Result<(), E>,
    {
        for item in self {
            callback(item.as_ref()).map_err(|e| e.with_pos(item.pos()))?;
        }

        Ok(())
    }
}
