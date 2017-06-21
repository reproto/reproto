use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct EnumVariant<'a> {
    pub name: AstLoc<&'a str>,
    pub comment: Vec<&'a str>,
    pub arguments: Vec<AstLoc<Value>>,
    pub ordinal: Option<AstLoc<Value>>,
}

/// enum value with assigned ordinal
impl<'a> IntoModel for (EnumVariant<'a>, u32) {
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
