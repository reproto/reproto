use super::error_pos::ErrorPos;
use std::result;

pub trait WithPos {
    /// Add additional position information, if it's not already present.
    fn with_pos<E: Into<ErrorPos>>(self, pos: E) -> Self;
}

impl<T, E> WithPos for result::Result<T, E>
where
    E: WithPos,
{
    fn with_pos<P: Into<ErrorPos>>(self, pos: P) -> Self {
        self.map_err(|e| e.with_pos(pos))
    }
}
