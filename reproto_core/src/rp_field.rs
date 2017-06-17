use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpField {
    pub modifier: RpModifier,
    pub name: String,
    pub comment: Vec<String>,
    #[serde(rename="type")]
    pub ty: RpType,
    pub field_as: Option<RpLoc<String>>,
}

impl RpField {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &str {
        if let Some(ref field) = self.field_as {
            &field.inner
        } else {
            &self.name
        }
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
                                                    f.pos.clone(),
                                                    field.pos.clone())
                    .into());
            }

            self.push(f);
        }

        Ok(())
    }
}
