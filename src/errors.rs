use parser::errors as parser;

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
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        Log(::log::SetLoggerError);
        Getopts(::getopts::Fail);
        ParseError(InternalError);
    }

    errors {
        MissingBackend {
        }
    }
}
