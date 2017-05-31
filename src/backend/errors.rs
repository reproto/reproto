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
            description("position error")
            display("{}", message)
        }

        FieldConflict(message: String, source: m::Pos, target: m::Pos) {
            description("field conflict")
            display("{}", message)
        }

        DeclMerge(message: String, source: m::Pos, target: m::Pos) {
            description("declaration merge")
            display("{}", message)
        }

        ExtendEnum(message: String, source: m::Pos, enum_pos: m::Pos) {
            description("extend enum")
            display("{}", message)
        }

        ReservedField(field_pos: m::Pos, reserved_pos: m::Pos) {
            description("field reserved")
            display("field reserved")
        }

        MatchConflict(source: m::Pos, target: m::Pos) {
            description("match conflict")
        }

        /// An instance creation is missing a set of required fields.
        MissingRequired(names: Vec<String>, pos: m::Pos, fields: Vec<m::Pos>) {
            description("missing required")
        }

        RegisteredTypeConflict {
        }

        /// Error thrown by Rc::get_mut
        RcGetMut {
        }

        RcTryUnwrap {
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

    pub fn extend_enum(message: String, source: m::Pos, enum_pos: m::Pos) -> Error {
        ErrorKind::ExtendEnum(message, source, enum_pos).into()
    }

    pub fn reserved_field(field_pos: m::Pos, reserved_pos: m::Pos) -> Error {
        ErrorKind::ReservedField(field_pos, reserved_pos).into()
    }
}
