use super::statement::Statement;
use super::name::{AsName, Name, ImportedName, BuiltInName};

#[derive(Debug, Clone)]
pub enum Variable {
    Literal(String),
    String(String),
    Statement(Statement),
    Name(Name),
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

impl AsVariable for Name {
    fn as_variable(self) -> Variable {
        Variable::Name(self)
    }
}

impl AsVariable for ImportedName {
    fn as_variable(self) -> Variable {
        Variable::Name(self.as_name())
    }
}

impl AsVariable for BuiltInName {
    fn as_variable(self) -> Variable {
        Variable::Name(self.as_name())
    }
}
