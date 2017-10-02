//! # Helper data structure do handle option lookups

use super::RpNumber;
use super::loc::Loc;
use super::option_entry::OptionEntry;
use super::pos::Pos;
use std::result;

type Result<T> = result::Result<T, (&'static str, Pos)>;

/// Helper for looking up and dealing with options.
pub trait Options {
    type Item: OptionEntry;

    fn items(&self) -> &Vec<Loc<Self::Item>>;

    fn lookup(&self, name: &str) -> Vec<Loc<&Self::Item>> {
        self.items()
            .iter()
            .filter(move |o| o.name() == name)
            .map(|option| option.loc_ref())
            .collect()
    }

    /// Find all strings matching the given name.
    ///
    /// This enforces that all found values are strings, otherwise the lookup will cause an error.
    fn find_all_strings(&self, name: &str) -> Result<Vec<Loc<String>>> {
        let mut out = Vec::new();

        for s in self.lookup(name) {
            let (value, pos) = s.take_pair();
            let string = value.as_string().map_err(|e| (e, pos.clone()))?;
            out.push(Loc::new(string, pos));
        }

        Ok(out)
    }

    fn find_all_u32(&self, name: &str) -> Result<Vec<Loc<RpNumber>>> {
        let mut out = Vec::new();

        for s in self.lookup(name) {
            let (value, pos) = s.take_pair();
            let number = value.as_number().map_err(|e| (e, pos.clone()))?;
            out.push(Loc::new(number, pos));
        }

        Ok(out)
    }

    /// Find all identifiers matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an error.
    fn find_all_identifiers(&self, name: &str) -> Result<Vec<Loc<String>>> {
        let mut out = Vec::new();

        for s in self.lookup(name) {
            let (value, pos) = s.take_pair();
            let identifier = value.as_identifier().map_err(|e| (e, pos.clone()))?;
            out.push(Loc::new(identifier, pos));
        }

        Ok(out)
    }

    /// Optionally find exactly one identifier matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an
    /// error.
    fn find_one_identifier(&self, name: &str) -> Result<Option<Loc<String>>> {
        Ok(self.find_all_identifiers(name)?.into_iter().next())
    }

    fn find_one_string(&self, name: &str) -> Result<Option<Loc<String>>> {
        Ok(self.find_all_strings(name)?.into_iter().next())
    }

    fn find_one_u32(&self, name: &str) -> Result<Option<Loc<RpNumber>>> {
        Ok(self.find_all_u32(name)?.into_iter().next())
    }
}

impl<T> Options for Vec<Loc<T>>
where
    T: OptionEntry,
{
    type Item = T;

    fn items(&self) -> &Vec<Loc<Self::Item>> {
        self
    }
}
