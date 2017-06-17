use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct MatchVariable {
    pub name: String,
    pub ty: RpType,
}

impl IntoModel for MatchVariable {
    type Output = RpMatchVariable;

    fn into_model(self, pos: &RpPos) -> Result<RpMatchVariable> {
        let match_variable = RpMatchVariable {
            name: self.name.into_model(pos)?,
            ty: self.ty,
        };

        Ok(match_variable)
    }
}
