use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpCode {
    pub context: String,
    pub lines: Vec<String>,
}

impl Merge for Vec<Loc<RpCode>> {
    fn merge(&mut self, source: Vec<Loc<RpCode>>) -> Result<()> {
        self.extend(source);
        Ok(())
    }
}
