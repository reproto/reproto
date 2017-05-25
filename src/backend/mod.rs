pub mod java;
pub mod python;

use environment::Environment;
use options::Options;
use parser::ast;
use std::path::PathBuf;
use std::fmt;

pub type TypeId = (ast::Package, String);

type Location = (PathBuf, usize, usize);

#[derive(Debug)]
enum ErrorCause {
    Message(String),
    IoError(::std::io::Error),
    Error(Box<::errors::Error>),
}

/// An error that occured at a given location.
#[derive(Debug)]
pub struct VerifyError {
    cause: ErrorCause,
    location: Option<Location>,
}

type Result<T> = ::std::result::Result<T, VerifyError>;

pub trait Backend {
    fn process(&self) -> Result<()>;

    fn verify(&self) -> Result<()>;
}

pub fn resolve(backend: &str, options: Options, env: Environment) -> Result<Box<Backend>> {
    let backend: Box<Backend> = match backend {
        "java" => Box::new(java::resolve(options, env)?),
        "python" => Box::new(python::resolve(options, env)?),
        _ => return Err(format!("Unknown backend type: {}", backend).into()),
    };

    Ok(backend)
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            ErrorCause::Message(ref string) => write!(f, "{}", string),
            ErrorCause::IoError(ref io) => io.fmt(f),
            ErrorCause::Error(ref error) => error.fmt(f),
        }
    }
}

impl ::std::error::Error for VerifyError {
    fn description(&self) -> &str {
        "backend error"
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match self.cause {
            ErrorCause::Message(_) => None,
            ErrorCause::IoError(ref io) => Some(io),
            ErrorCause::Error(ref error) => Some(error),
        }
    }
}

impl From<String> for VerifyError {
    fn from(value: String) -> VerifyError {
        VerifyError {
            cause: ErrorCause::Message(value),
            location: None,
        }
    }
}

impl<'a> From<&'a str> for VerifyError {
    fn from(value: &'a str) -> VerifyError {
        VerifyError {
            cause: ErrorCause::Message(value.to_owned()),
            location: None,
        }
    }
}

impl From<::std::io::Error> for VerifyError {
    fn from(value: ::std::io::Error) -> VerifyError {
        VerifyError {
            cause: ErrorCause::IoError(value),
            location: None,
        }
    }
}

impl From<::errors::Error> for VerifyError {
    fn from(value: ::errors::Error) -> VerifyError {
        VerifyError {
            cause: ErrorCause::Error(Box::new(value)),
            location: None,
        }
    }
}
