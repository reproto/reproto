use std::path::PathBuf;

use parser::errors as parser;
use codeviz::errors as codeviz;

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
        Codeviz(codeviz::Error, codeviz::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Log(::log::SetLoggerError);
        ParseError(InternalError);
    }

    errors {
        MissingBackend {
        }

        DeclError(path: PathBuf, line_string: String, line: usize, decl: ast::Decl) {
            description("Error in declaration")
            display("Error in declaration `{}`: {}:{}: `{}`", decl.display(), path.display(), line, line_string)
        }

        DeclConflict(path: PathBuf, line_string: String, line: usize, existing: ast::Decl, conflicting: ast::Decl) {
            description("Conflicting type declared")
            display("Conflicting type declared: {}:{}: `{}`", path.display(), line, line_string)
        }
    }
}
