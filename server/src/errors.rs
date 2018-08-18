use core::errors as core;
use std::borrow::Cow;

/// Service errors.
pub enum Error {
    NotFound,
    BadRequest(Cow<'static, str>),
    InternalServerError(Cow<'static, str>),
    Core(core::Error),
}

impl<T> From<T> for Error
where
    T: Into<core::Error>,
{
    fn from(value: T) -> Error {
        Error::Core(value.into())
    }
}
