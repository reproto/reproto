use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct Field<'input> {
    pub modifier: RpModifier,
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub ty: RpType,
    pub field_as: Option<AstLoc<Value<'input>>>,
}

impl<'input> Field<'input> {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }
}

impl<'input> IntoModel for Field<'input> {
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

        let name = self.name.to_owned();
        let comment = self.comment.into_iter().map(ToOwned::to_owned).collect();
        Ok(RpField::new(self.modifier, name, comment, self.ty, field_as))
    }
}
