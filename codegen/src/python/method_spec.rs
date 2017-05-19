use super::decorator_spec::{AsDecoratorSpec, DecoratorSpec};
use super::element_spec::AsElementSpec;
use super::elements::Elements;
use super::statement::{AsStatement, Statement};

#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub name: String,
    pub decorators: Vec<DecoratorSpec>,
    pub arguments: Vec<Statement>,
    pub elements: Elements,
}

impl MethodSpec {
    pub fn new(name: &str) -> MethodSpec {
        MethodSpec {
            name: name.to_owned(),
            decorators: Vec::new(),
            arguments: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_decorator<D>(&mut self, decorator: D)
        where D: AsDecoratorSpec
    {
        self.decorators.push(decorator.as_decorator_spec());
    }

    pub fn push_argument<S>(&mut self, argument: S)
        where S: AsStatement
    {
        self.arguments.push(argument.as_statement());
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }
}
