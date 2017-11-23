use core::{ErrorPos, Pos, Reporter, RpType, WithPos, errors as core};
use parser::errors as parser;
use repository::errors as repository;
use serde_json as json;

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Core(core::Error, core::ErrorKind);
        Repository(repository::Error, repository::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Fmt(::std::fmt::Error);
        Json(json::Error);
        BorrowMutError(::std::cell::BorrowMutError);
        BorrowError(::std::cell::BorrowError);
    }

    errors {
        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        Errors(errors: Vec<Error>) {
            description("errors")
            display("encountered {} error(s)", errors.len())
        }

        MissingOutput {
            description("missing output directory, use --out or the `output` key in the manifest")
        }

        MissingPrefix(prefix: String) {
            description("missing prefix")
            display("missing prefix: {}", prefix)
        }

        MissingTypeImpl(ty: RpType, suggestion: &'static str) {
            description("missing type implementation")
            display("missing implementation for type `{}`, {}", ty, suggestion)
        }
    }
}

impl Error {
    pub fn pos(message: String, pos: ErrorPos) -> Error {
        ErrorKind::Pos(message, pos).into()
    }
}

impl WithPos for Error {
    fn with_pos<E: Into<ErrorPos>>(self, pos: E) -> Self {
        use self::ErrorKind::*;

        match self.kind() {
            &Pos(..) => self,
            &Errors(..) => self,
            _ => {
                let message = format!("{}", self);
                self.chain_err(|| ErrorKind::Pos(message, pos.into()))
            }
        }
    }
}

impl<'a> From<(&'a str, Pos)> for Error {
    fn from(value: (&'a str, Pos)) -> Self {
        ErrorKind::Pos(value.0.to_string(), value.1.into()).into()
    }
}

impl<'a> From<Reporter<'a>> for Error {
    fn from(value: Reporter<'a>) -> Self {
        let e: core::Error = value.into();
        e.into()
    }
}
