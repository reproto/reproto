#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Unexpected { pos: usize },
}

pub type Result<T> = ::std::result::Result<T, Error>;
