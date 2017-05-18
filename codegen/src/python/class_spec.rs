use super::element_spec::{AsElementSpec, ElementSpec};
use super::decorator_spec::{AsDecoratorSpec, DecoratorSpec};

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub name: String,
    pub decorators: Vec<DecoratorSpec>,
    pub elements: Vec<ElementSpec>,
}

impl ClassSpec {
    pub fn new(name: &str) -> ClassSpec {
        ClassSpec {
            name: name.to_owned(),
            decorators: Vec::new(),
            elements: Vec::new(),
        }
    }

    pub fn push_decorator<D>(&mut self, decorator: D)
        where D: AsDecoratorSpec
    {
        self.decorators.push(decorator.as_decorator_spec());
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element.as_element_spec());
    }
}
