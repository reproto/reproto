use reproto_core::ErrorPos;
use reproto_core::errors as core;

error_chain! {
    links {
        Core(core::Error, core::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseFloat(::std::num::ParseFloatError);
        FromUtf8Error(::std::string::FromUtf8Error);
        ParseBigIntError(::num::bigint::ParseBigIntError);
        BorrowMutError(::std::cell::BorrowMutError);
        BorrowError(::std::cell::BorrowError);
    }

    errors {
        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        FieldConflict(message: String, source: ErrorPos, target: ErrorPos) {
            description("field conflict")
            display("{}", message)
        }

        EnumVariantConflict(pos: ErrorPos, other: ErrorPos) {
            description("enum value conflict")
        }

        Syntax(pos: Option<ErrorPos>, expected: Vec<String>) {
            description("syntax error")
        }

        Parse(message: &'static str, pos: ErrorPos) {
            description("parse error")
            display("parse error: {}", message)
        }

        Overflow(pos: ErrorPos) {
        }
    }
}
