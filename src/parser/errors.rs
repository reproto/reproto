use super::ast;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }

    errors {
        InvalidEscape {
        }

        InvalidMerge(this: ast::Decl, other: ast::Decl) {
            description("Invalid merge")
            display("Cannot merge existing `{}` with `{}`", this.display(), other.display())
        }

        Syntax(message: String, line_string: String, line: usize) {
            description("Syntax error")
            display("Syntax error: {}: {}", message, line_string)
        }
    }
}
