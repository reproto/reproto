use parser::ast;
use super::*;
use super::errors::*;
use super::into_model::IntoModel;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct RpInstance {
    pub name: RpName,
    pub arguments: RpLoc<Vec<RpLoc<RpFieldInit>>>,
}

impl IntoModel for ast::Instance {
    type Output = RpInstance;

    fn into_model(self, pos: &RpPos) -> Result<RpInstance> {
        let instance = RpInstance {
            name: self.name,
            arguments: self.arguments.into_model(pos)?,
        };

        Ok(instance)
    }
}
