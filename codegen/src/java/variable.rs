use super::_type::{AsType, Type, ClassType};
use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::field_spec::FieldSpec;
use super::modifier::Modifiers;
use super::statement::{AsStatement, Statement};

#[derive(Debug, Clone)]
pub enum Variable {
    Literal(String),
    Type(Type),
    String(String),
    Statement(Statement),
    Spacing,
}

pub trait AsVariable {
    fn as_variable(self) -> Variable;
}

impl<'a, A> AsVariable for &'a A
    where A: AsVariable + Clone
{
    fn as_variable(self) -> Variable {
        self.clone().as_variable()
    }
}

impl AsVariable for Variable {
    fn as_variable(self) -> Variable {
        self
    }
}

impl<'a> AsVariable for &'a str {
    fn as_variable(self) -> Variable {
        Variable::Literal(self.to_owned())
    }
}

impl AsVariable for String {
    fn as_variable(self) -> Variable {
        Variable::Literal(self)
    }
}

impl AsVariable for Statement {
    fn as_variable(self) -> Variable {
        Variable::Statement(self)
    }
}

impl AsVariable for FieldSpec {
    fn as_variable(self) -> Variable {
        Variable::Literal(self.name)
    }
}

impl AsVariable for ArgumentSpec {
    fn as_variable(self) -> Variable {
        Variable::Literal(self.name)
    }
}

impl AsVariable for Modifiers {
    fn as_variable(self) -> Variable {
        Variable::Literal(self.format())
    }
}

impl AsVariable for Type {
    fn as_variable(self) -> Variable {
        Variable::Type(self)
    }
}

impl AsVariable for ClassType {
    fn as_variable(self) -> Variable {
        Variable::Type(self.as_type())
    }
}

impl AsVariable for AnnotationSpec {
    fn as_variable(self) -> Variable {
        Variable::Statement(self.as_statement())
    }
}
