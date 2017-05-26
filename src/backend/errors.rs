use super::models::Pos;

#[derive(Debug)]
pub struct EnvironmentError {
    pub message: String,
    pub pos: Pos,
}

impl EnvironmentError {
    pub fn new(message: String, pos: Pos) -> EnvironmentError {
        EnvironmentError {
            message: message,
            pos: pos,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Message(String),
    Pos(String, Pos),
    DeclMerge(String, Pos, Pos),
    FieldConflict(String, Pos, Pos),
    Error(Box<::errors::Error>),
}

impl Error {
    pub fn pos(message: String, pos: Pos) -> Error {
        Error::Pos(message, pos)
    }

    pub fn field_conflict(field: String, source: Pos, target: Pos) -> Error {
        Error::FieldConflict(field, source, target)
    }

    pub fn decl_merge(message: String, source: Pos, target: Pos) -> Error {
        Error::DeclMerge(message, source, target)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Message(ref message) => write!(f, "{}", message),
            Error::Pos(ref message, _) => write!(f, "{}", message),
            Error::Error(ref error) => error.fmt(f),
            Error::FieldConflict(ref name, _, _) => write!(f, "conflicting field: {}", name),
            Error::DeclMerge(ref message, _, _) => write!(f, "{}", message),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "backend error"
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Error(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Error {
        Error::Message(value)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(value: &'a str) -> Error {
        Error::Message(value.to_owned())
    }
}

impl From<::std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Error {
        Error::Error(Box::new(value.into()))
    }
}

impl From<::errors::Error> for Error {
    fn from(value: ::errors::Error) -> Error {
        Error::Error(Box::new(value))
    }
}
