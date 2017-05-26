error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        FromUtf8Error(::std::string::FromUtf8Error);
    }

    errors {
        InvalidEscape {
        }

        Syntax(message: String, line_string: String, line: usize) {
            description("Syntax error")
            display("Syntax error line {}: {}: {}", line, message, line_string)
        }
    }
}
