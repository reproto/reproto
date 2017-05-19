use super::annotation_spec::AnnotationSpec;
use super::element_spec::AsElementSpec;
use super::elements::Elements;
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct InterfaceSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub elements: Elements,
}

impl InterfaceSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> InterfaceSpec {
        InterfaceSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element)
    }
}
