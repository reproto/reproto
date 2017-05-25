pub mod java;
pub mod python;

use environment::Environment;
use options::Options;
use ast;
use std::fmt;
use std::path::PathBuf;

pub type TypeId = (ast::Package, String);

#[derive(Debug)]
pub struct VerifyError {
    pub message: String,
    pub path: PathBuf,
    pub location: Location,
}

impl VerifyError {
    pub fn new(message: String, path: PathBuf, location: Location) -> VerifyError {
        VerifyError {
            message: message,
            path: path,
            location: location,
        }
    }
}

type Location = (usize, usize);

#[derive(Debug)]
pub enum Error {
    Message(String),
    Location(String, Location),
    Error(Box<::errors::Error>),
}

impl Error {
    pub fn location(message: String, location: Location) -> Error {
        Error::Location(message, location)
    }
}

type Result<T> = ::std::result::Result<T, Error>;

pub trait Backend {
    fn process(&self) -> Result<()>;

    fn verify(&self) -> Result<Vec<VerifyError>>;
}

pub fn resolve(backend: &str, options: Options, env: Environment) -> Result<Box<Backend>> {
    let backend: Box<Backend> = match backend {
        "java" => Box::new(java::resolve(options, env)?),
        "python" => Box::new(python::resolve(options, env)?),
        _ => return Err(format!("Unknown backend type: {}", backend).into()),
    };

    Ok(backend)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Message(ref message) => write!(f, "{}", message),
            Error::Location(ref message, _) => write!(f, "{}", message),
            Error::Error(ref error) => error.fmt(f),
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
