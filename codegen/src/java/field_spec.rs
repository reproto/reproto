use super::modifier::Modifiers;
use super::_type::{AsType, Type};

#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub modifiers: Modifiers,
    pub ty: Type,
    pub name: String,
}

impl FieldSpec {
    pub fn new<I>(modifiers: Modifiers, ty: I, name: &str) -> FieldSpec
        where I: AsType
    {
        FieldSpec {
            modifiers: modifiers,
            ty: ty.as_type(),
            name: name.to_owned(),
        }
    }
}

pub trait AsFieldSpec {
    fn as_field_spec(self) -> FieldSpec;
}

impl<'a, A> AsFieldSpec for &'a A
    where A: AsFieldSpec + Clone
{
    fn as_field_spec(self) -> FieldSpec {
        self.clone().as_field_spec()
    }
}

impl AsFieldSpec for FieldSpec {
    fn as_field_spec(self) -> FieldSpec {
        self
    }
}
