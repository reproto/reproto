error_chain! {
    errors {
        Syntax(pos: Option<(usize, usize)>, expected: Vec<String>) {
            description("syntax error")
        }

        Parse(pos: Option<(usize, usize)>, message: &'static str) {
            description("parse error")
            display("parse error: {}", message)
        }
    }
}
