use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct Field {
    pub modifier: RpModifier,
    pub name: String,
    pub comment: Vec<String>,
    pub ty: RpType,
    pub field_as: Option<AstLoc<Value>>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }
}

impl IntoModel for Field {
    type Output = RpField;

    fn into_model(self, path: &Path) -> Result<RpField> {
        let field_as = self.field_as.into_model(path)?;

        let field_as = if let Some(field_as) = field_as {
            match field_as.both() {
                (RpValue::String(name), pos) => Some(RpLoc::new(name, pos)),
                (_, pos) => return Err(ErrorKind::Pos("must be a string".to_owned(), pos).into()),
            }
        } else {
            None
        };

        Ok(RpField::new(self.modifier, self.name, self.comment, self.ty, field_as))
    }
}
