use reqwest;
use reqwest::header::parsing;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
  ReqwestError(reqwest::Error),
  UrlError(reqwest::UrlError),
  FormatError(fmt::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Error::ReqwestError(value)
  }
}

impl From<reqwest::UrlError> for Error {
  fn from(value: reqwest::UrlError) -> Self {
    Error::UrlError(value)
  }
}

impl From<fmt::Error> for Error {
  fn from(value: fmt::Error) -> Self {
    Error::FormatError(value)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::ReqwestError(ref e) => e.fmt(fmt),
      Error::UrlError(ref e) => e.fmt(fmt),
      Error::FormatError(ref e) => e.fmt(fmt),
    }
  }
}

pub struct PathEncode<T>(pub T);

impl<T> fmt::Display for PathEncode<T>
where
  T: fmt::Display
{
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    parsing::http_percent_encode(fmt, self.0.to_string().as_bytes())
  }
}
