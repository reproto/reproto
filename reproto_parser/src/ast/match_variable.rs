use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct MatchVariable {
    pub name: String,
    pub ty: RpType,
}

impl IntoModel for MatchVariable {
    type Output = RpMatchVariable;

    fn into_model(self, path: &Path) -> Result<RpMatchVariable> {
        let match_variable = RpMatchVariable {
            name: self.name.into_model(path)?,
            ty: self.ty,
        };

        Ok(match_variable)
    }
}
