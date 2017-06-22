use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Instance<'input> {
    pub name: RpName,
    pub arguments: AstLoc<Vec<AstLoc<FieldInit<'input>>>>,
}

impl<'input> IntoModel for Instance<'input> {
    type Output = RpInstance;

    fn into_model(self, path: &Path) -> Result<RpInstance> {
        let instance = RpInstance {
            name: self.name,
            arguments: self.arguments.into_model(path)?,
        };

        Ok(instance)
    }
}
