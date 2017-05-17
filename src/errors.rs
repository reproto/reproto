use std::path::PathBuf;

use parser::errors as parser;
use codegen::errors as codegen;

use parser::ast;

#[derive(Debug)]
pub enum InternalError {
    ParseError,
}

impl ::std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Debug::fmt(self, f)
    }
}

impl ::std::error::Error for InternalError {
    fn description(&self) -> &str {
        "Internal Error"
    }
}

error_chain! {
    links {
        Parser(parser::Error, parser::ErrorKind);
        Codegen(codegen::Error, codegen::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        Log(::log::SetLoggerError);
        Getopts(::getopts::Fail);
        ParseError(InternalError);
    }

    errors {
        MissingBackend {
        }

        ConflictingTypeDecl(path: PathBuf, line_string: String, line: usize, existing: ast::Decl, conflicting: ast::Decl) {
            description("Conflicting type declared")
            display("Conflicting type declared: {}:{}: {}", path.display(), line, line_string)
        }
    }
}
