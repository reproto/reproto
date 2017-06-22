use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct EnumVariant<'input> {
    pub name: AstLoc<&'input str>,
    pub comment: Vec<&'input str>,
    pub arguments: Vec<AstLoc<Value<'input>>>,
    pub ordinal: Option<AstLoc<Value<'input>>>,
}

/// enum value with assigned ordinal
impl<'input> IntoModel for (EnumVariant<'input>, u32) {
    type Output = Rc<RpEnumVariant>;

    fn into_model(self, path: &Path) -> Result<Self::Output> {
        let value = self.0;
        let ordinal = self.1;

        let value = RpEnumVariant {
            name: value.name.into_model(path)?,
            comment: value.comment.into_iter().map(ToOwned::to_owned).collect(),
            arguments: value.arguments.into_model(path)?,
            ordinal: ordinal,
        };

        Ok(Rc::new(value))
    }
}
