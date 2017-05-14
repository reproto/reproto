error_chain! {
    foreign_links {
        ParseInt(::std::num::ParseIntError);
    }

    errors {
        InvalidEscape {
        }

        Syntax(s: String) {
            description("syntax error")
            display("syntax error: {}", s)
        }
    }
}
