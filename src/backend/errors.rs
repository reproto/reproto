use codeviz::errors as codeviz;
use core::*;
use core::errors as core;
use parser::errors as parser;
use serde_json as json;

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Core(core::Error, core::ErrorKind);
        Codeviz(codeviz::Error, codeviz::ErrorKind);
    }

    foreign_links {
        IO(::std::io::Error);
        Fmt(::std::fmt::Error);
        Json(json::Error);
    }

    errors {
        Pos(message: String, pos: RpPos) {
            description("position error")
            display("{}", message)
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
