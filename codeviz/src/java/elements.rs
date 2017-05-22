use super::element_spec::{AsElementSpec, ElementSpec};

#[derive(Debug, Clone)]
pub struct Elements {
    pub elements: Vec<ElementSpec>,
}

impl Elements {
    pub fn new() -> Elements {
        Elements { elements: Vec::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element.as_element_spec());
    }

    pub fn push_nested<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(ElementSpec::Nested(Box::new(element.as_element_spec())));
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn join<S>(self, separator: S) -> Elements
        where S: AsElementSpec + Clone
    {
        let mut it = self.elements.into_iter();

        let part = match it.next() {
            Some(part) => part,
            None => return Elements::new(),
        };

        let mut parts: Elements = Elements::new();
        parts.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            parts.push(sep.as_element_spec());
            parts.push(part);
        }

        parts
    }
}
