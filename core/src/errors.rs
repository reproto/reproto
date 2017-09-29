use super::ErrorPos;
use extern_mime;

error_chain! {
    foreign_links {
        IO(::std::io::Error);
        Fmt(::std::fmt::Error);
    }

    errors {
        MimeFromStrError(error: extern_mime::FromStrError) {
            description("couldn't parse mime type")
            display("{:?}", error)
        }

        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        DeclMerge(message: String, source: ErrorPos, target: ErrorPos) {
            description("declaration merge")
            display("{}", message)
        }

        FieldConflict(message: String, source: ErrorPos, target: ErrorPos) {
            description("field conflict")
            display("{}", message)
        }

        ExtendEnum(message: String, source: ErrorPos, enum_pos: ErrorPos) {
            description("extend enum")
            display("{}", message)
        }

        ReservedField(field_pos: ErrorPos, reserved_pos: ErrorPos) {
            description("field reserved")
            display("field reserved")
        }

        MatchConflict(source: ErrorPos, target: ErrorPos) {
            description("match conflict")
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
