use Source;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize)]
pub struct Span {
    #[serde(skip)]
    pub source: Arc<Source>,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn empty() -> Span {
        Span {
            source: Arc::new(Source::empty("empty")),
            start: 0,
            end: 0,
        }
    }
}

impl<'a> From<&'a Span> for Span {
    fn from(value: &'a Span) -> Span {
        Span {
            source: Arc::clone(&value.source),
            start: value.start,
            end: value.end,
        }
    }
}

impl From<(Arc<Source>, usize, usize)> for Span {
    fn from(value: (Arc<Source>, usize, usize)) -> Self {
        Span {
            source: value.0,
            start: value.1,
            end: value.2,
        }
    }
}
