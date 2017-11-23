use backend::errors as backend;
use core::{ErrorPos, RpPackage};
use core::errors as core;
use manifest::errors as manifest;
use parser::errors as parser;
use repository::errors as repository;
use semck::Violation;
use std::path::PathBuf;

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Core(core::Error, core::ErrorKind);
        Repository(repository::Error, repository::ErrorKind);
        Backend(backend::Error, backend::ErrorKind);
        Manifest(manifest::Error, manifest::ErrorKind);
    }

    foreign_links {
        BorrowMutError(::std::cell::BorrowMutError);
        Io(::std::io::Error);
        Fmt(::std::fmt::Error);
        Log(::log::SetLoggerError);
        Toml(::toml::de::Error);
        UrlParseError(::url::ParseError);
        FromUtf8Error(::std::string::FromUtf8Error);
        BorrowError(::std::cell::BorrowError);
    }

    errors {
        Pos(message: String, pos: ErrorPos) {
            description("position error")
            display("{}", message)
        }

        SemckViolation(index: usize, violation: Violation) {
            description("semck violation")
        }

        File(message: String, file: PathBuf) {
            description("file error")
            display("{}: {}", file.display(), message)
        }

        Errors(errors: Vec<Error>) {
            description("errors")
            display("encountered {} error(s)", errors.len())
        }

        MissingBackend {
        }

        NoVersionToPublish(package: RpPackage) {
            description("no version to publish")
            display("no version to publish: {}", package)
        }

        PoisonError {
            description("poison error")
        }
    }
}

impl Error {
    pub fn pos(message: String, pos: ErrorPos) -> Error {
        ErrorKind::Pos(message, pos).into()
    }
}
