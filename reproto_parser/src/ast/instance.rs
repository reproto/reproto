use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Instance<'input> {
    pub name: RpName,
    pub arguments: AstLoc<'input, Vec<AstLoc<'input, FieldInit<'input>>>>,
}

impl<'input> IntoModel for Instance<'input> {
    type Output = RpInstance;

    fn into_model(self) -> Result<RpInstance> {
        let instance = RpInstance {
            name: self.name,
            arguments: self.arguments.into_model()?,
        };

        Ok(instance)
    }
}
