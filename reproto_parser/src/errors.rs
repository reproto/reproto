use reproto_core::RpPos;
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
        Pos(message: String, pos: RpPos) {
            description("position error")
            display("{}", message)
        }

        FieldConflict(message: String, source: RpPos, target: RpPos) {
            description("field conflict")
            display("{}", message)
        }

        EnumVariantConflict(pos: RpPos, other: RpPos) {
            description("enum value conflict")
        }

        Syntax(pos: Option<RpPos>, expected: Vec<String>) {
            description("syntax error")
        }

        Parse(message: &'static str, pos: RpPos) {
            description("parse error")
            display("parse error: {}", message)
        }

        Overflow(pos: RpPos) {
        }
    }
}
