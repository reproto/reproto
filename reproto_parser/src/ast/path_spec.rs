use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct PathSpec<'input> {
    pub segments: Vec<PathSegment<'input>>,
}

impl<'input> IntoModel for PathSpec<'input> {
    type Output = RpPathSpec;

    fn into_model(self) -> Result<RpPathSpec> {
        Ok(RpPathSpec { segments: self.segments.into_model()? })
    }
}
