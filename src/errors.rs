use reproto_backend::errors as backend;
use reproto_core::{ErrorPos, RpTypeId};
use reproto_core::errors as core;
use reproto_parser::errors as parser;
use reproto_repository::errors as repository;

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Core(core::Error, core::ErrorKind);
        Repository(repository::Error, repository::ErrorKind);
        Backend(backend::Error, backend::ErrorKind);
    }

    foreign_links {
        BorrowMutError(::std::cell::BorrowMutError);
        Io(::std::io::Error);
        Fmt(::std::fmt::Error);
        Log(::log::SetLoggerError);
        Toml(::toml::de::Error);
        UrlParseError(::url::ParseError);
    }

    errors {
        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        Errors(errors: Vec<Error>) {
            description("errors")
            display("encountered {} error(s)", errors.len())
        }

        MissingBackend {
        }

        /// An instance creation is missing a set of required fields.
        MissingRequired(names: Vec<String>, pos: ErrorPos, fields: Vec<ErrorPos>) {
            description("missing required")
        }

        RegisteredTypeConflict(type_id: RpTypeId) {
            description("registered type conflict")
            display("registered type conflict with: {}", type_id)
        }
    }
}

impl Error {
    pub fn pos(message: String, pos: ErrorPos) -> Error {
        ErrorKind::Pos(message, pos).into()
    }
}
