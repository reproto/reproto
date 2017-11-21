//! Path specifications

use super::RpPathSegment;

#[derive(Debug, Clone, Serialize)]
pub struct RpPathSpec {
    pub segments: Vec<RpPathSegment>,
}

impl RpPathSpec {
    pub fn url(&self) -> String {
        let segments: Vec<String> = self.segments.iter().map(RpPathSegment::path).collect();
        format!("/{}", segments.join("/"))
    }

    pub fn id_fragments(&self) -> Vec<&str> {
        self.segments.iter().map(RpPathSegment::id).collect()
    }
}
