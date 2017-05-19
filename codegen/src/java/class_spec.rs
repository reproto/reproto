use super::annotation_spec::AnnotationSpec;
use super::constructor_spec::{AsConstructorSpec, ConstructorSpec};
use super::element_spec::AsElementSpec;
use super::elements::Elements;
use super::field_spec::{AsFieldSpec, FieldSpec};
use super::modifier::Modifiers;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub fields: Vec<FieldSpec>,
    pub constructors: Vec<ConstructorSpec>,
    pub elements: Elements,
}

impl ClassSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> ClassSpec {
        ClassSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            elements: Elements::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push_field<F>(&mut self, field: F)
        where F: AsFieldSpec
    {
        self.fields.push(field.as_field_spec());
    }

    pub fn push_constructor<C>(&mut self, constructor: C)
        where C: AsConstructorSpec
    {
        self.constructors.push(constructor.as_constructor_spec());
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }
}
