use std::collections::HashSet;
use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RpModifiers {
    pub modifiers: HashSet<RpModifier>,
}

impl RpModifiers {
    pub fn new(modifiers: HashSet<RpModifier>) -> RpModifiers {
        RpModifiers { modifiers: modifiers }
    }

    pub fn test(&self, modifier: &RpModifier) -> bool {
        self.modifiers.contains(modifier)
    }
}
