use super::_type::{AsType, Type};
use super::annotation_spec::{AsAnnotationSpec, AnnotationSpec};
use super::argument_spec::{AsArgumentSpec, ArgumentSpec};
use super::element_spec::AsElementSpec;
use super::elements::Elements;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub returns: Option<Type>,
    pub elements: Elements,
}

impl MethodSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> MethodSpec {
        MethodSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            arguments: Vec::new(),
            returns: None,
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

    pub fn returns<T>(&mut self, returns: T)
        where T: AsType
    {
        self.returns = Some(returns.as_type())
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }
}
