use super::*;

pub struct RpPath<'input> {
    pub parts: Vec<RpPathFragment<'input>>,
}
