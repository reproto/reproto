#[derive(Debug)]
pub enum Error {
    UnterminatedString { start: usize },
    UnterminatedEscape { start: usize },
    InvalidEscape { message: &'static str, pos: usize },
    UnterminatedCodeBlock { start: usize },
    InvalidNumber { message: &'static str, pos: usize },
    Unexpected { pos: usize },
}

pub type Result<T> = ::std::result::Result<T, Error>;
