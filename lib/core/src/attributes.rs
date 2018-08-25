//! Provide a collection of attributes and a convenient way for querying them.
//!
//! These structures are all map-like.

use errors::Result;
use serde::Serialize;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use {Diagnostics, Flavor, Loc, RpValue, Span, Translate, Translator};

#[derive(Debug, Clone, Serialize)]
#[serde(bound = "F::Package: Serialize")]
pub struct Selection<F: 'static>
where
    F: Flavor,
{
    /// Storing words and their locations.
    words: Vec<Loc<RpValue<F>>>,
    /// Storing values and their locations.
    values: HashMap<String, (Loc<String>, Loc<RpValue<F>>)>,
}

impl<F: 'static> Selection<F>
where
    F: Flavor,
{
    pub fn new(
        words: Vec<Loc<RpValue<F>>>,
        values: HashMap<String, (Loc<String>, Loc<RpValue<F>>)>,
    ) -> Selection<F> {
        Selection { words, values }
    }

    /// Take the given value, removing it in the process.
    pub fn take<Q: ?Sized>(&mut self, key: &Q) -> Option<Loc<RpValue<F>>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.values.remove(key).map(|v| v.1)
    }

    /// Take the given value, removing it in the process.
    pub fn take_words(&mut self) -> Vec<Loc<RpValue<F>>> {
        mem::replace(&mut self.words, vec![])
    }

    /// Take a single word.
    pub fn take_word(&mut self) -> Option<Loc<RpValue<F>>> {
        self.words.pop()
    }

    /// Get an iterator over unused positions.
    pub fn unused(&self) -> impl Iterator<Item = Span> {
        let mut positions = Vec::new();
        positions.extend(self.words.iter().map(|v| Loc::span(v)));
        positions.extend(self.values.values().map(|v| Loc::span(&v.0)));
        positions.into_iter()
    }
}

impl<F: 'static, T> Translate<T> for Selection<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = Selection<T::Target>;

    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Selection<T::Target>> {
        Ok(Selection {
            words: self.words.translate(diag, translator)?,
            values: self.values.translate(diag, translator)?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(bound = "F::Package: Serialize")]
pub struct Attributes<F: 'static>
where
    F: Flavor,
{
    words: HashMap<String, Span>,
    selections: HashMap<String, Loc<Selection<F>>>,
}

impl<F: 'static> Attributes<F>
where
    F: Flavor,
{
    pub fn new(
        words: HashMap<String, Span>,
        selections: HashMap<String, Loc<Selection<F>>>,
    ) -> Attributes<F> {
        Attributes { words, selections }
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
    pub fn take_selection<Q: ?Sized>(&mut self, key: &Q) -> Option<Loc<Selection<F>>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.selections.remove(key)
    }

    /// Get an iterator over unused positions.
    pub fn unused(&self) -> impl Iterator<Item = Span> {
        let mut positions = Vec::new();
        positions.extend(self.words.values());
        positions.extend(self.selections.values().map(Loc::span));
        positions.into_iter()
    }
}

impl<F: 'static, T> Translate<T> for Attributes<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = Attributes<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Attributes<T::Target>> {
        Ok(Attributes {
            words: self.words,
            selections: self.selections.translate(diag, translator)?,
        })
    }
}
