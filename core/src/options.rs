//! # Helper data structure do handle option lookups

use super::*;
use super::errors::*;

/// Helper for looking up and dealing with options.
#[derive(Debug, Clone, Serialize)]
pub struct Options {
    options: Vec<Loc<RpOptionDecl>>,
}

impl Options {
    pub fn new(options: Vec<Loc<RpOptionDecl>>) -> Options {
        Options { options: options }
    }

    pub fn lookup<'a>(&'a self, name: &'a str) -> Box<Iterator<Item = &Loc<RpValue>> + 'a> {
        let it = self.options.iter();

        Box::new(it.filter(move |o| o.name.as_str() == name).flat_map(|o| {
            o.values.iter()
        }))
    }

    pub fn find_one<'a>(&'a self, name: &'a str) -> Result<Option<&'a Loc<RpValue>>> {
        let mut it = self.lookup(name);

        if let Some(next) = it.next() {
            if let Some(s) = it.next() {
                return Err(
                    ErrorKind::Pos(
                        format!("{}: only one value may be present", name),
                        s.pos().into(),
                    ).into(),
                );
            }

            return Ok(Some(next));
        }

        Ok(None)
    }

    /// Find all strings matching the given name.
    ///
    /// This enforces that all found values are strings, otherwise the lookup will cause an error.
    pub fn find_all_strings(&self, name: &str) -> Result<Vec<Loc<String>>> {
        let mut out: Vec<Loc<String>> = Vec::new();

        for s in self.lookup(name) {
            match s.as_ref_pair() {
                (&RpValue::String(ref string), ref pos) => {
                    out.push(Loc::new(string.clone(), (*pos).clone()));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos(format!("{}: expected string", name), (*pos).into())
                            .into(),
                    );
                }
            }
        }

        Ok(out)
    }

    pub fn find_all_numbers(&self, name: &str) -> Result<Vec<Loc<RpNumber>>> {
        let mut out: Vec<Loc<RpNumber>> = Vec::new();

        for s in self.lookup(name) {
            match s.as_ref_pair() {
                (&RpValue::Number(ref number), ref pos) => {
                    out.push(Loc::new(number.clone(), (*pos).clone()));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos(format!("{}: expected number", name), (*pos).into())
                            .into(),
                    );
                }
            }
        }

        Ok(out)
    }

    /// Optionally find exactly one identifier matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an
    /// error.
    pub fn find_one_identifier(&self, name: &str) -> Result<Option<Loc<String>>> {
        if let Some(t) = self.find_one(name)? {
            match t.as_ref_pair() {
                (&RpValue::Identifier(ref identifier), ref pos) => {
                    return Ok(Some(Loc::new(identifier.clone(), (*pos).clone())));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos("expected identifier".to_owned(), (*pos).into()).into(),
                    );
                }
            }
        }

        Ok(None)
    }

    pub fn find_one_string(&self, name: &str) -> Result<Option<Loc<String>>> {
        if let Some(t) = self.find_one(name)? {
            match t.as_ref_pair() {
                (&RpValue::String(ref string), ref pos) => {
                    return Ok(Some(Loc::new(string.to_owned(), (*pos).clone())));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos("expected string".to_owned(), (*pos).into()).into(),
                    );
                }
            }
        }

        Ok(None)
    }

    pub fn find_one_number(&self, name: &str) -> Result<Option<Loc<RpNumber>>> {
        if let Some(t) = self.find_one(name)? {
            match t.as_ref_pair() {
                (&RpValue::Number(ref number), ref pos) => {
                    return Ok(Some(Loc::new(number.clone(), (*pos).clone())));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos("expected number".to_owned(), (*pos).into()).into(),
                    );
                }
            }
        }

        Ok(None)
    }

    pub fn find_one_boolean(&self, name: &str) -> Result<Option<Loc<bool>>> {
        if let Some(t) = self.find_one(name)? {
            match t.as_ref_pair() {
                (&RpValue::Boolean(ref boolean), ref pos) => {
                    return Ok(Some(Loc::new(boolean.clone(), (*pos).clone())));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos("expected boolean".to_owned(), (*pos).into()).into(),
                    );
                }
            }
        }

        Ok(None)
    }

    /// Find all identifiers matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an error.
    pub fn find_all_identifiers(&self, name: &str) -> Result<Vec<Loc<String>>> {
        let mut out: Vec<Loc<String>> = Vec::new();

        for s in self.lookup(name) {
            match s.as_ref_pair() {
                (&RpValue::Identifier(ref identifier), ref pos) => {
                    out.push(Loc::new(identifier.clone(), (*pos).clone()));
                }
                (_, ref pos) => {
                    return Err(
                        ErrorKind::Pos(format!("{}: expected identifier", name), (*pos).into())
                            .into(),
                    );
                }
            }
        }

        Ok(out)
    }
}
