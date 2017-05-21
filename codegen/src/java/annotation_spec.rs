use super::_type::{AsType, Type, ClassType};
use super::statement::{AsStatement, Statement};

#[derive(Debug, Clone)]
pub struct AnnotationSpec {
    pub ty: Type,
    pub arguments: Vec<Statement>,
}

impl AnnotationSpec {
    pub fn new<I>(ty: I) -> AnnotationSpec
        where I: AsType
    {
        AnnotationSpec {
            ty: ty.as_type(),
            arguments: Vec::new(),
        }
    }

    pub fn push_argument<S>(&mut self, statement: S)
        where S: AsStatement
    {
        self.arguments.push(statement.as_statement());
    }
}

pub trait AsAnnotationSpec {
    fn as_annotation_spec(self) -> AnnotationSpec;
}

impl<'a, A> AsAnnotationSpec for &'a A
    where A: AsAnnotationSpec + Clone
{
    fn as_annotation_spec(self) -> AnnotationSpec {
        self.clone().as_annotation_spec()
    }
}

impl AsAnnotationSpec for AnnotationSpec {
    fn as_annotation_spec(self) -> AnnotationSpec {
        self
    }
}

impl AsAnnotationSpec for ClassType {
    fn as_annotation_spec(self) -> AnnotationSpec {
        AnnotationSpec::new(self)
    }
}
