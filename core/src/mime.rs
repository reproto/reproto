use super::errors::*;
use extern_mime;
use serde;
use std::fmt;
use std::result;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Mime(extern_mime::Mime);

impl serde::Serialize for Mime {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

impl FromStr for Mime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Mime(s.parse().map_err(ErrorKind::MimeFromStrError)?))
    }
}

impl fmt::Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
