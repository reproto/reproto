use super::name::{AsName, Name, BuiltInName, ImportedName};
use super::statement::{AsStatement, Statement};

#[derive(Debug, Clone)]
pub struct DecoratorSpec {
    pub name: Name,
    pub arguments: Vec<Statement>,
}

impl DecoratorSpec {
    pub fn new<N>(name: N) -> DecoratorSpec
        where N: AsName
    {
        DecoratorSpec {
            name: name.as_name(),
            arguments: Vec::new(),
        }
    }

    pub fn push_argument<S>(&mut self, statement: S)
        where S: AsStatement
    {
        self.arguments.push(statement.as_statement());
    }
}

pub trait AsDecoratorSpec {
    fn as_decorator_spec(self) -> DecoratorSpec;
}

impl<'a, A> AsDecoratorSpec for &'a A
    where A: AsDecoratorSpec + Clone
{
    fn as_decorator_spec(self) -> DecoratorSpec {
        self.clone().as_decorator_spec()
    }
}

impl AsDecoratorSpec for DecoratorSpec {
    fn as_decorator_spec(self) -> DecoratorSpec {
        self
    }
}

impl AsDecoratorSpec for BuiltInName {
    fn as_decorator_spec(self) -> DecoratorSpec {
        DecoratorSpec::new(self.as_name())
    }
}

impl AsDecoratorSpec for ImportedName {
    fn as_decorator_spec(self) -> DecoratorSpec {
        DecoratorSpec::new(self.as_name())
    }
}
