use std::result;

pub enum Error {
  Unknown,
}

pub type Result<T> = result::Result<T, Error>;

impl<T> From<T> for Error {
  fn from(value: T) -> Self {
    Error::Unknown
  }
}
