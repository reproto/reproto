use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpField {
    pub modifier: RpModifier,
    name: String,
    pub comment: Vec<String>,
    #[serde(rename="type")]
    pub ty: RpType,
    pub field_as: Option<RpLoc<String>>,
}

impl RpField {
    pub fn new(modifier: RpModifier,
               name: String,
               comment: Vec<String>,
               ty: RpType,
               field_as: Option<RpLoc<String>>)
               -> RpField {
        RpField {
            modifier: modifier,
            name: name,
            comment: comment,
            ty: ty,
            field_as: field_as,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }

    pub fn ident(&self) -> &str {
        &self.name
    }

    pub fn name(&self) -> &str {
        self.field_as.as_ref().map(AsRef::as_ref).unwrap_or(&self.name)
    }

    pub fn display(&self) -> String {
        self.name.to_owned()
    }
}

impl Merge for Vec<RpLoc<RpField>> {
    fn merge(&mut self, source: Vec<RpLoc<RpField>>) -> Result<()> {
        for f in source {
            if let Some(field) = self.iter().find(|e| e.name == f.name) {
                return Err(ErrorKind::FieldConflict(f.name.clone(),
                                                    f.pos().into(),
                                                    field.pos().into())
                    .into());
            }

            self.push(f);
        }

        Ok(())
    }
}
