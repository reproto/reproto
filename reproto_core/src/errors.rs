use super::{RpPos, RpTypeId};

error_chain! {
    foreign_links {
        IO(::std::io::Error);
        Fmt(::std::fmt::Error);
    }

    errors {
        DeclMerge(message: String, source: RpPos, target: RpPos) {
            description("declaration merge")
            display("{}", message)
        }

        FieldConflict(message: String, source: RpPos, target: RpPos) {
            description("field conflict")
            display("{}", message)
        }

        ExtendEnum(message: String, source: RpPos, enum_pos: RpPos) {
            description("extend enum")
            display("{}", message)
        }

        ReservedField(field_pos: RpPos, reserved_pos: RpPos) {
            description("field reserved")
            display("field reserved")
        }

        MatchConflict(source: RpPos, target: RpPos) {
            description("match conflict")
        }

        /// An instance creation is missing a set of required fields.
        MissingRequired(names: Vec<String>, pos: RpPos, fields: Vec<RpPos>) {
            description("missing required")
        }

        RegisteredTypeConflict(type_id: RpTypeId) {
            description("registered type conflict")
            display("registered type conflict with: {:?}", type_id)
        }

        /// Error thrown by Rc::get_mut
        RcGetMut {
        }

        RcTryUnwrap {
        }

        Overflow {
        }
    }
}
