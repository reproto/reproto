//! # Helper data structure do handle option lookups

use super::*;
use super::errors::*;

/// Helper for looking up and dealing with options.
#[derive(Debug)]
pub struct Options<'a> {
    pos: &'a RpPos,
    options: Vec<RpLoc<RpOptionDecl>>,
}

impl<'a> Options<'a> {
    pub fn new(pos: &'a RpPos, options: Vec<RpLoc<RpOptionDecl>>) -> Options {
        Options {
            pos: pos,
            options: options,
        }
    }

    pub fn lookup(&'a self, name: &'a str) -> Box<Iterator<Item = &RpLoc<RpValue>> + 'a> {
        let it = self.options
            .iter();

        Box::new(it.filter(move |o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter()))
    }

    /// Find all strings matching the given name.
    ///
    /// This enforces that all found values are strings, otherwise the lookup will cause an error.
    pub fn find_all_strings(&self, name: &str) -> Result<Vec<RpLoc<String>>> {
        let mut out: Vec<RpLoc<String>> = Vec::new();

        for s in self.lookup(name) {
            match s.ref_both() {
                (&RpValue::String(ref string), ref pos) => {
                    out.push(RpLoc::new(string.clone(), (*pos).clone()));
                }
                (_, ref pos) => {
                    return Err(ErrorKind::Pos(format!("{}: expected string", name),
                                              (*pos).clone())
                        .into());
                }
            }
        }

        Ok(out)
    }

    pub fn find_one(&'a self, name: &'a str) -> Result<Option<&'a RpLoc<RpValue>>> {
        let mut out: Option<&RpLoc<RpValue>> = None;

        for s in self.lookup(name) {
            if let Some(_) = out {
                return Err(ErrorKind::Pos(format!("{}: only one value may be present", name),
                                          s.pos().clone())
                    .into());
            }

            out = Some(s);
        }

        Ok(out)
    }

    /// Optionally find exactly one identifier matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an
    /// error.
    pub fn find_one_identifier(&self, name: &str) -> Result<Option<RpLoc<String>>> {
        if let Some(t) = self.find_one(name)? {
            match t.ref_both() {
                (&RpValue::Identifier(ref identifier), ref pos) => {
                    return Ok(Some(RpLoc::new(identifier.clone(), (*pos).clone())));
                }
                (_, ref pos) => {
                    return Err(ErrorKind::Pos("expected identifier".to_owned(), (*pos).clone())
                        .into());
                }
            }
        }

        Ok(None)
    }

    pub fn find_one_boolean(&self, name: &str) -> Result<Option<RpLoc<bool>>> {
        if let Some(t) = self.find_one(name)? {
            match t.ref_both() {
                (&RpValue::Boolean(ref boolean), ref pos) => {
                    return Ok(Some(RpLoc::new(boolean.clone(), (*pos).clone())));
                }
                (_, ref pos) => {
                    return Err(ErrorKind::Pos("expected boolean".to_owned(), (*pos).clone())
                        .into());
                }
            }
        }

        Ok(None)
    }

    /// Find all identifiers matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an error.
    pub fn find_all_identifiers(&self, name: &str) -> Result<Vec<RpLoc<String>>> {
        let mut out: Vec<RpLoc<String>> = Vec::new();

        for s in self.lookup(name) {
            match s.ref_both() {
                (&RpValue::Identifier(ref identifier), ref pos) => {
                    out.push(RpLoc::new(identifier.clone(), (*pos).clone()));
                }
                (_, ref pos) => {
                    return Err(ErrorKind::Pos(format!("{}: expected identifier", name),
                                              (*pos).clone())
                        .into());
                }
            }
        }

        Ok(out)
    }
}
