//! Provide a collection of attributes and a convenient way for querying them.
//!
//! These structures are all map-like.

use loc::Loc;
use pos::Pos;
use rp_value::RpValue;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::vec;

/// Iterator over unused positions.
pub struct Unused<'a> {
    iter: vec::IntoIter<&'a Pos>,
}

impl<'a> Iterator for Unused<'a> {
    type Item = &'a Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Selection {
    /// Storing words and their locations.
    words: HashMap<String, Pos>,
    /// Storing values and their locations.
    values: HashMap<String, (Loc<String>, Loc<RpValue>)>,
}

impl Selection {
    pub fn new(
        words: HashMap<String, Pos>,
        values: HashMap<String, (Loc<String>, Loc<RpValue>)>,
    ) -> Selection {
        Selection {
            words: words,
            values: values,
        }
    }

    /// Take the given value, removing it in the process.
    pub fn take<Q: ?Sized>(&mut self, key: &Q) -> Option<Loc<RpValue>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.values.remove(key).map(|v| v.1)
    }

    /// Get an iterator over unused positions.
    pub fn unused(&self) -> Unused {
        let mut positions = Vec::new();
        positions.extend(self.words.values());
        positions.extend(self.values.values().map(|v| v.0.pos()));
        Unused { iter: positions.into_iter() }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Attributes {
    words: HashMap<String, Pos>,
    selections: HashMap<String, Loc<Selection>>,
}

impl Attributes {
    pub fn new(
        words: HashMap<String, Pos>,
        selections: HashMap<String, Loc<Selection>>,
    ) -> Attributes {
        Attributes {
            words: words,
            selections: selections,
        }
    }

    /// Take the given selection, removing it in the process.
    pub fn take_word<Q: ?Sized>(&mut self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.words.remove(key).is_some()
    }

    /// Take the given selection, removing it in the process.
    pub fn take_selection<Q: ?Sized>(&mut self, key: &Q) -> Option<Loc<Selection>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.selections.remove(key)
    }

    /// Get an iterator over unused positions.
    pub fn unused(&self) -> Unused {
        let mut positions = Vec::new();
        positions.extend(self.words.values());
        positions.extend(self.selections.values().map(Loc::pos));
        Unused { iter: positions.into_iter() }
    }
}
