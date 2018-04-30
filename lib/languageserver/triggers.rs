//! Data structure to handle location triggers.
use core::Position;
use models::Range;
use std::collections::{BTreeMap, Bound};
use ty;

#[derive(Debug, Clone)]
pub struct Triggers<T> {
    triggers: BTreeMap<Position, (Range, T)>,
}

impl<T> Triggers<T> {
    /// Create a new trigger container.
    pub fn new() -> Self {
        Self {
            triggers: BTreeMap::new(),
        }
    }

    /// Insert the given trigger.
    pub fn insert(&mut self, range: Range, value: T) {
        self.triggers.insert(range.start, (range, value));
    }

    /// Test if the given position matches a trigger in the containers.
    pub fn find(&self, position: ty::Position) -> Option<&T> {
        use self::Bound::*;

        let end = Position {
            line: position.line as usize,
            col: position.character as usize,
        };

        let mut range = self.triggers.range((Unbounded, Included(&end)));

        let (range, value) = match range.next_back() {
            Some((_, &(ref range, ref value))) => (range, value),
            None => return None,
        };

        if !range.contains(&end) {
            return None;
        }

        Some(value)
    }
}
