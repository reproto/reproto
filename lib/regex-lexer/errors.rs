use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Unexpected { pos: usize },
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "path parser error")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "path parser error"
    }
}
