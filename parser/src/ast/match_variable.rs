use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct MatchVariable<'input> {
    pub name: &'input str,
    pub ty: RpType,
}

impl<'input> IntoModel for MatchVariable<'input> {
    type Output = RpMatchVariable;

    fn into_model(self) -> Result<RpMatchVariable> {
        let match_variable = RpMatchVariable {
            name: self.name.into_model()?,
            ty: self.ty,
        };

        Ok(match_variable)
    }
}
