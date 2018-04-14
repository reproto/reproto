use Source;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize)]
pub struct Pos {
    #[serde(skip)]
    pub source: Arc<Source>,
    pub start: usize,
    pub end: usize,
}

impl Pos {
    pub fn empty() -> Pos {
        Pos {
            source: Arc::new(Source::empty("empty")),
            start: 0,
            end: 0,
        }
    }
}

impl<'a> From<&'a Pos> for Pos {
    fn from(value: &'a Pos) -> Pos {
        Pos {
            source: Arc::clone(&value.source),
            start: value.start,
            end: value.end,
        }
    }
}

impl From<(Arc<Source>, usize, usize)> for Pos {
    fn from(value: (Arc<Source>, usize, usize)) -> Self {
        Pos {
            source: value.0,
            start: value.1,
            end: value.2,
        }
    }
}
