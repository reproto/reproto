use super::_type::{AsType, Type};
use super::annotation_spec::AnnotationSpec;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct ArgumentSpec {
    pub modifiers: Modifiers,
    pub ty: Type,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
}

impl ArgumentSpec {
    pub fn new<I>(modifiers: Modifiers, ty: I, name: &str) -> ArgumentSpec
        where I: AsType
    {
        ArgumentSpec {
            modifiers: modifiers,
            ty: ty.as_type(),
            name: name.to_owned(),
            annotations: Vec::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }
}

pub trait AsArgumentSpec {
    fn as_argument_spec(self) -> ArgumentSpec;
}

impl<'a, A> AsArgumentSpec for &'a A
    where A: AsArgumentSpec + Clone
{
    fn as_argument_spec(self) -> ArgumentSpec {
        self.clone().as_argument_spec()
    }
}

impl AsArgumentSpec for ArgumentSpec {
    fn as_argument_spec(self) -> ArgumentSpec {
        self
    }
}
