use std::collections::BTreeMap;
use std::collections::btree_map;
use super::errors::*;
use super::models::*;

/// Adds the into_model() method for all types that supports ConvertToModel.
pub trait Merge {
    /// Convert the current type to a model.
    fn merge(&mut self, other: Self) -> Result<()>;
}

impl<T> Merge for Token<T>
    where T: Merge
{
    fn merge(&mut self, source: Token<T>) -> Result<()> {
        self.inner.merge(source.inner)?;
        Ok(())
    }
}

impl Merge for SubType {
    fn merge(&mut self, source: SubType) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        self.names.extend(source.names);
        Ok(())
    }
}

impl Merge for InterfaceBody {
    fn merge(&mut self, source: InterfaceBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        self.sub_types.merge(source.sub_types)?;
        Ok(())
    }
}

impl Merge for TypeBody {
    fn merge(&mut self, source: TypeBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        Ok(())
    }
}

impl Merge for TupleBody {
    fn merge(&mut self, source: TupleBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        Ok(())
    }
}

impl Merge for EnumBody {
    fn merge(&mut self, source: EnumBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        Ok(())
    }
}

impl Merge for Vec<Token<Code>> {
    fn merge(&mut self, source: Vec<Token<Code>>) -> Result<()> {
        self.extend(source);
        Ok(())
    }
}

impl Merge for Vec<Token<Field>> {
    fn merge(&mut self, source: Vec<Token<Field>>) -> Result<()> {
        for f in source {
            if let Some(field) = self.iter().find(|e| e.name == f.name) {
                return Err(Error::field_conflict(field.name.clone(),
                                                 field.pos.clone(),
                                                 f.pos.clone()));
            }

            self.push(f);
        }

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
