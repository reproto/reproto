use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub name: RpName,
    pub arguments: AstLoc<Vec<AstLoc<FieldInit>>>,
}

impl IntoModel for Instance {
    type Output = RpInstance;

    fn into_model(self, path: &Path) -> Result<RpInstance> {
        let instance = RpInstance {
            name: self.name,
            arguments: self.arguments.into_model(path)?,
        };

        Ok(instance)
    }
}
