//! Models used to structure regular expressions.

use std::fmt;

/// A regular expression.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Regex {
    pub parts: Vec<Item>,
}

impl fmt::Display for Regex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.parts {
            p.fmt(fmt)?;
        }

        Ok(())
    }
}

/// A single statement in a regular expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// ```ignore
    /// <item>*
    /// ```
    ZeroOrMore { item: Box<Item> },
    /// ```ignore
    /// <item>+
    /// ```
    OneOrMore { item: Box<Item> },
    /// ```ignore
    /// <item>?
    /// ```
    Optional { item: Box<Item> },
    /// ```ignore
    /// [..]
    /// ```
    CharacterClass { character_class: CharacterClass },
    /// A single character.
    Character { character: char },
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Item::*;

        match *self {
            ZeroOrMore { ref item } => write!(fmt, "{}*", item),
            OneOrMore { ref item } => write!(fmt, "{}+", item),
            Optional { ref item } => write!(fmt, "{}?", item),
            CharacterClass {
                ref character_class,
            } => character_class.fmt(fmt),
            Character { ref character } => character.fmt(fmt),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A single character constraint.
pub enum CharacterConstraint {
    /// A single matching character.
    Character { character: char },
    /// ```ignore
    /// <start>-<end>
    /// ```
    Range { start: char, end: char },
}

impl fmt::Display for CharacterConstraint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::CharacterConstraint::*;

        match *self {
            Character { ref character } => write!(fmt, "{}", character),
            Range { ref start, ref end } => write!(fmt, "{}-{}", start, end),
        }
    }
}

/// A characterclass that defines a set of constraints over a single character.
///
/// ```ignore
/// [<constraints>]
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CharacterClass {
    pub constraints: Vec<CharacterConstraint>,
}

impl fmt::Display for CharacterClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[")?;

        for c in &self.constraints {
            c.fmt(fmt)?;
        }

        write!(fmt, "]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut regex = Regex::default();
        let mut character_class = CharacterClass::default();

        character_class
            .constraints
            .push(CharacterConstraint::Range {
                start: 'a',
                end: 'z',
            });

        regex.parts.push(Item::ZeroOrMore {
            item: Box::new(Item::CharacterClass {
                character_class: character_class.clone(),
            }),
        });

        regex.parts.push(Item::OneOrMore {
            item: Box::new(Item::CharacterClass {
                character_class: character_class.clone(),
            }),
        });

        regex.parts.push(Item::Optional {
            item: Box::new(Item::CharacterClass {
                character_class: character_class.clone(),
            }),
        });

        assert_eq!(String::from("[a-z]*[a-z]+[a-z]?"), regex.to_string());
    }
}
