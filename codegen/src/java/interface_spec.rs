use super::annotation_spec::AnnotationSpec;
use super::class_spec::ClassSpec;
use super::element_spec::ElementSpec;
use super::modifier::Modifiers;
use super::statement::AsStatement;

#[derive(Debug, Clone)]
pub struct InterfaceSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub elements: Vec<ElementSpec>,
}

impl InterfaceSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> InterfaceSpec {
        InterfaceSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            elements: Vec::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push_class(&mut self, class: &ClassSpec) {
        self.elements.push(ElementSpec::Class(class.clone()))
    }

    pub fn push_interface(&mut self, interface: &InterfaceSpec) {
        self.elements.push(ElementSpec::Interface(interface.clone()))
    }

    pub fn push_statement<S>(&mut self, statement: S)
        where S: AsStatement
    {
        self.elements.push(ElementSpec::Statement(statement.as_statement()))
    }

    pub fn push_literal(&mut self, content: &Vec<String>) {
        self.elements.push(ElementSpec::Literal(content.clone()))
    }
}
