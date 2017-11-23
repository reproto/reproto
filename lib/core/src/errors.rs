use super::ErrorPos;
use super::with_pos::WithPos;
use extern_mime;

error_chain! {
    foreign_links {
        IO(::std::io::Error);
        Fmt(::std::fmt::Error);
    }

    errors {
        Context {
            description("context error")
            display("context error")
        }

        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        MimeFromStrError(error: extern_mime::FromStrError) {
            description("couldn't parse mime type")
            display("{:?}", error)
        }

        InvalidOrdinal {
        }

        /// Error thrown by Rc::get_mut
        RcGetMut {
        }

        RcTryUnwrap {
        }
    }
}

impl WithPos for Error {
    fn with_pos<E: Into<ErrorPos>>(self, pos: E) -> Self {
        use self::ErrorKind::*;

        match self.kind() {
            &Pos(..) => self,
            _ => {
                let message = format!("{}", &self);
                self.chain_err(|| ErrorKind::Pos(message, pos.into()))
            }
        }
    }
}
