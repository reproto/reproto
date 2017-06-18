use reproto_core::RpPos;
use reproto_core::errors as core;
use reproto_core::semver;

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
        ReqParseError(semver::ReqParseError);
        SemVerError(semver::SemVerError);
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

        InvalidEscape {
        }

        Syntax(pos: Option<RpPos>, expected: Vec<String>) {
            description("syntax error")
        }

        Overflow {
        }

        IllegalToken(pos: usize) {
            description("illegal token")
        }
    }
}
