use super::*;
use super::errors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Object<'input> {
    Instance(Loc<Instance<'input>>),
    Constant(Loc<RpName>),
}

impl<'input> IntoModel for Object<'input> {
    type Output = RpObject;

    fn into_model(self) -> Result<RpObject> {
        use self::Object::*;

        let out = match self {
            Instance(instance) => RpObject::Instance(instance.into_model()?),
            Constant(constant) => RpObject::Constant(constant),
        };

        Ok(out)
    }
}
