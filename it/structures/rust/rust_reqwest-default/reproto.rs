use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
  ReqwestError(reqwest::Error),
  UrlParseError(url::ParseError),
  FormatError(fmt::Error)
}

pub type Result<T, E = Error> = result::Result<T, E>;

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Error::ReqwestError(value)
  }
}

impl From<url::ParseError> for Error {
  fn from(value: url::ParseError) -> Self {
    Error::UrlParseError(value)
  }
}

impl From<fmt::Error> for Error {
  fn from(value: fmt::Error) -> Self {
    Error::FormatError(value)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::ReqwestError(e) => e.fmt(fmt),
      Error::UrlParseError(e) => e.fmt(fmt),
      Error::FormatError(e) => e.fmt(fmt),
    }
  }
}

pub struct PathEncode<T>(pub T);

impl<T> fmt::Display for PathEncode<T>
where
  T: fmt::Display
{
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "{}", percent_encoding::utf8_percent_encode(&self.0.to_string(), percent_encoding::NON_ALPHANUMERIC))
  }
}
