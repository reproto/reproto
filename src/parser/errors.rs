use core::*;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseFloat(::std::num::ParseFloatError);
        ParseBigInt(::num::bigint::ParseBigIntError);
        FromUtf8Error(::std::string::FromUtf8Error);
    }

    errors {
        InvalidEscape {
        }

        Syntax(pos: Option<RpPos>, expected: Vec<String>) {
            description("syntax error")
        }

        Overflow {
        }

        IllegalToken {
        }
    }
}
