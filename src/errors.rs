use codeviz::errors as codeviz;
use reproto_core::{RpPos, RpTypeId};
use reproto_core::errors as core;
use reproto_parser::errors as parser;
use reproto_repository::errors as repository;
use serde_json as json;

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Core(core::Error, core::ErrorKind);
        Codeviz(codeviz::Error, codeviz::ErrorKind);
        Repository(repository::Error, repository::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Fmt(::std::fmt::Error);
        Log(::log::SetLoggerError);
        Json(json::Error);
    }

    errors {
        Pos(message: String, pos: RpPos) {
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
        MissingRequired(names: Vec<String>, pos: RpPos, fields: Vec<RpPos>) {
            description("missing required")
        }

        RegisteredTypeConflict(type_id: RpTypeId) {
            description("registered type conflict")
            display("registered type conflict with: {:?}", type_id)
        }
    }
}

impl Error {
    pub fn pos(message: String, pos: RpPos) -> Error {
        ErrorKind::Pos(message, pos).into()
    }
}
