use Span;
use std::result;

pub trait WithSpan {
    /// Add additional position information, if it's not already present.
    fn with_span<E: Into<Span>>(self, span: E) -> Self;
}

impl<T, E> WithSpan for result::Result<T, E>
where
    E: WithSpan,
{
    fn with_span<P: Into<Span>>(self, span: P) -> Self {
        self.map_err(|e| e.with_span(span))
    }
}
