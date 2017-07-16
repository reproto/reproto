use std::collections::BTreeMap;
use std::collections::btree_map;
use std::rc::Rc;
use super::errors::*;

/// Merging of models.
pub trait Merge {
    /// Merge the current model with another.
    fn merge(&mut self, other: Self) -> Result<()>;
}

impl<T> Merge for Rc<T>
    where T: Merge
{
    fn merge(&mut self, source: Rc<T>) -> Result<()> {
        let mut rc = Rc::get_mut(self).ok_or(ErrorKind::RcGetMut)?;
        let source = Rc::try_unwrap(source).map_err(|_| ErrorKind::RcTryUnwrap)?;
        rc.merge(source)?;
        Ok(())
    }
}

impl<K, T> Merge for BTreeMap<K, T>
    where T: Merge,
          K: ::std::cmp::Ord
{
    fn merge(&mut self, source: BTreeMap<K, T>) -> Result<()> {
        for (key, value) in source {
            match self.entry(key) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(value);
                }
                btree_map::Entry::Occupied(entry) => {
                    Merge::merge(entry.into_mut(), value)?;
                }
            }
        }

        Ok(())
    }
}
