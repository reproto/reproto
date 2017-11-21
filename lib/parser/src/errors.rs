use core::{ErrorPos, errors as core};
use std::path::PathBuf;

error_chain! {
    links {
        Core(core::Error, core::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseFloat(::std::num::ParseFloatError);
        ParseBigIntError(::num::bigint::ParseBigIntError);
    }

    errors {
        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        File(message: String, file: PathBuf) {
            description("file error")
            display("{}: {}", file.display(), message)
        }

        Syntax(pos: Option<ErrorPos>, expected: Vec<String>) {
            description("syntax error")
        }

        Parse(message: &'static str, pos: ErrorPos) {
            description("parse error")
            display("parse error: {}", message)
        }
    }
}
