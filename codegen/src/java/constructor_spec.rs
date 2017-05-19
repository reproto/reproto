use super::annotation_spec::{AsAnnotationSpec, AnnotationSpec};
use super::argument_spec::{AsArgumentSpec, ArgumentSpec};
use super::element_spec::{AsElementSpec, ElementSpec};
use super::elements::Elements;
use super::modifier::Modifiers;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct ConstructorSpec {
    pub modifiers: Modifiers,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub elements: Elements,
}

impl ConstructorSpec {
    pub fn new(modifiers: Modifiers) -> ConstructorSpec {
        ConstructorSpec {
            modifiers: modifiers,
            annotations: Vec::new(),
            arguments: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_annotation<A>(&mut self, annotation: A)
        where A: AsAnnotationSpec
    {
        self.annotations.push(annotation.as_annotation_spec());
    }

    pub fn push_argument<A>(&mut self, argument: A)
        where A: AsArgumentSpec
    {
        self.arguments.push(argument.as_argument_spec());
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }

    pub fn as_element_spec(&self, enclosing: &str) -> ElementSpec {
        let mut elements = Elements::new();

        let mut open = Statement::new();

        for a in &self.annotations {
            elements.push(a);
        }

        if !self.modifiers.is_empty() {
            open.push(&self.modifiers);
            open.push(" ");
        }

        open.push(enclosing);
        open.push("(");
        open.push(Statement::join_statements(&self.arguments, ", "));
        open.push(") {");

        elements.push(open);
        elements.push_nested(&self.elements);
        elements.push("}");

        elements.as_element_spec()
    }
}

pub trait AsConstructorSpec {
    fn as_constructor_spec(self) -> ConstructorSpec;
}

impl<'a, A> AsConstructorSpec for &'a A
    where A: AsConstructorSpec + Clone
{
    fn as_constructor_spec(self) -> ConstructorSpec {
        self.clone().as_constructor_spec()
    }
}

impl AsConstructorSpec for ConstructorSpec {
    fn as_constructor_spec(self) -> ConstructorSpec {
        self
    }
}
