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

    fn into_model(self, pos: &RpPos) -> Result<RpField> {
        let field_as = self.field_as.into_model(pos)?;

        let field_as = if let Some(field_as) = field_as {
            if let RpValue::String(name) = field_as.inner {
                Some(RpLoc::new(name, field_as.pos.clone()))
            } else {
                return Err(ErrorKind::Pos("must be a string".to_owned(), field_as.pos).into());
            }
        } else {
            None
        };

        let field = RpField {
            modifier: self.modifier,
            name: self.name,
            comment: self.comment,
            ty: self.ty,
            field_as: field_as,
        };

        Ok(field)
    }
}
