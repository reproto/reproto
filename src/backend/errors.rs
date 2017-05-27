use super::models as m;
use parser::errors as parser_errors;

error_chain! {
    links {
        Parser(parser_errors::Error, parser_errors::ErrorKind);
    }

    foreign_links {
        IO(::std::io::Error);
    }

    errors {
        Pos(message: String, pos: m::Pos) {
        }

        FieldConflict(message: String, source: m::Pos, target: m::Pos) {
        }

        DeclMerge(message: String, source: m::Pos, target: m::Pos) {
        }
    }
}

impl Error {
    pub fn pos(message: String, pos: m::Pos) -> Error {
        ErrorKind::Pos(message, pos).into()
    }

    pub fn field_conflict(field: String, source: m::Pos, target: m::Pos) -> Error {
        ErrorKind::FieldConflict(field, source, target).into()
    }

    pub fn decl_merge(message: String, source: m::Pos, target: m::Pos) -> Error {
        ErrorKind::DeclMerge(message, source, target).into()
    }
}
