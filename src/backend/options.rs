use super::errors::*;
use super::models::*;

/// Helper for looking up and dealing with options.
#[derive(Debug)]
pub struct Options<'a> {
    pos: &'a Pos,
    options: Vec<Token<OptionDecl>>,
}

impl<'a> Options<'a> {
    pub fn new(pos: &'a Pos, options: Vec<Token<OptionDecl>>) -> Options {
        Options {
            pos: pos,
            options: options,
        }
    }

    pub fn lookup(&'a self, name: &'a str) -> Box<Iterator<Item = &Token<Value>> + 'a> {
        let it = self.options
            .iter();

        Box::new(it.filter(move |o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter()))
    }

    /// Find all strings matching the given name.
    ///
    /// This enforces that all found values are strings, otherwise the lookup will cause an error.
    pub fn find_all_strings(&self, name: &str) -> Result<Vec<Token<String>>> {
        let mut out: Vec<Token<String>> = Vec::new();

        for s in self.lookup(name) {
            match **s {
                Value::String(ref string) => {
                    out.push(Token::new(string.clone(), s.pos.clone()));
                }
                _ => {
                    return Err(Error::pos(format!("{}: expected string", name), s.pos.clone()));
                }
            }
        }

        Ok(out)
    }

    pub fn find_one(&'a self, name: &'a str) -> Result<Option<&'a Token<Value>>> {
        let mut out: Option<&Token<Value>> = None;

        for s in self.lookup(name) {
            if let Some(_) = out {
                return Err(Error::pos(format!("{}: only one value may be present", name),
                                      s.pos.clone()));
            }

            out = Some(s);
        }

        Ok(out)
    }

    /// Optionally find exactly one identifier matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an
    /// error.
    pub fn find_one_identifier(&self, name: &str) -> Result<Option<Token<String>>> {
        if let Some(t) = self.find_one(name)? {
            if let Value::Identifier(ref identifier) = t.inner {
                return Ok(Some(Token::new(identifier.clone(), t.pos.clone())));
            } else {
                return Err(Error::pos("expected identifier".to_owned(), t.pos.clone()));
            }
        }

        Ok(None)
    }

    pub fn _find_one_string(&self, name: &str) -> Result<Option<Token<String>>> {
        if let Some(t) = self.find_one(name)? {
            if let Value::String(ref string) = t.inner {
                return Ok(Some(Token::new(string.clone(), t.pos.clone())));
            } else {
                return Err(Error::pos("expected string".to_owned(), t.pos.clone()));
            }
        }

        Ok(None)
    }

    pub fn find_one_boolean(&self, name: &str) -> Result<Option<Token<bool>>> {
        if let Some(t) = self.find_one(name)? {
            if let Value::Boolean(ref boolean) = t.inner {
                return Ok(Some(Token::new(boolean.clone(), t.pos.clone())));
            } else {
                return Err(Error::pos("expected string".to_owned(), t.pos.clone()));
            }
        }

        Ok(None)
    }

    /// Find all identifiers matching the given name.
    ///
    /// This enforces that all found values are identifiers, otherwise the lookup will cause an error.
    pub fn find_all_identifiers(&self, name: &str) -> Result<Vec<Token<String>>> {
        let mut out: Vec<Token<String>> = Vec::new();

        for s in self.lookup(name) {
            match **s {
                Value::Identifier(ref identifier) => {
                    out.push(Token::new(identifier.clone(), s.pos.clone()));
                }
                _ => {
                    return Err(Error::pos(format!("{}: expected identifier", name), s.pos.clone()));
                }
            }
        }

        Ok(out)
    }
}
