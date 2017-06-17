use backend::errors as backend;
use codeviz::errors as codeviz;
use reproto_core::errors as core;
use reproto_parser::errors as parser;

#[derive(Debug)]
pub enum InternalError {
    ParseError,
}

impl ::std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Debug::fmt(self, f)
    }
}

impl ::std::error::Error for InternalError {
    fn description(&self) -> &str {
        "Internal Error"
    }
}

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Core(core::Error, core::ErrorKind);
        Codeviz(codeviz::Error, codeviz::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Log(::log::SetLoggerError);
        ParseError(InternalError);
        BackendError(backend::Error);
    }

    errors {
        BackendErrors(errors: Vec<backend::Error>) {
            description("backend errors")
            display("encountered {} backend error(s)", errors.len())
        }

        MissingBackend {
        }
    }
}

impl From<Vec<backend::Error>> for Error {
    fn from(errors: Vec<backend::Error>) -> Error {
        ErrorKind::BackendErrors(errors).into()
    }
}
