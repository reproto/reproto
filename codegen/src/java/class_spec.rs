use super::annotation_spec::AnnotationSpec;
use super::constructor_spec::ConstructorSpec;
use super::element_spec::ElementSpec;
use super::field_spec::FieldSpec;
use super::interface_spec::InterfaceSpec;
use super::method_spec::MethodSpec;
use super::modifier::Modifiers;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct ClassSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub fields: Vec<FieldSpec>,
    pub constructors: Vec<ConstructorSpec>,
    pub methods: Vec<MethodSpec>,
    pub elements: Vec<ElementSpec>,
}

impl ClassSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> ClassSpec {
        ClassSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
            elements: Vec::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push_field(&mut self, field: &FieldSpec) {
        self.fields.push(field.clone());
    }

    pub fn push_constructor(&mut self, constructor: &ConstructorSpec) {
        self.constructors.push(constructor.clone());
    }

    pub fn push_method(&mut self, method: &MethodSpec) {
        self.methods.push(method.clone());
    }

    pub fn push_class(&mut self, class: &ClassSpec) {
        self.elements.push(ElementSpec::Class(class.clone()))
    }

    pub fn push_interface(&mut self, interface: &InterfaceSpec) {
        self.elements.push(ElementSpec::Interface(interface.clone()))
    }

    pub fn push_statement(&mut self, statement: &Statement) {
        self.elements.push(ElementSpec::Statement(statement.clone()))
    }

    pub fn push_literal(&mut self, content: &Vec<String>) {
        self.elements.push(ElementSpec::Literal(content.clone()))
    }
}
