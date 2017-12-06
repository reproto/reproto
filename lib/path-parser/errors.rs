use path_lexer;

#[derive(Debug)]
pub enum Error {
    Lexer(path_lexer::Error),
    Syntax(Option<(usize, usize)>, Vec<String>),
    Parse(Option<(usize, usize)>, &'static str),
}
