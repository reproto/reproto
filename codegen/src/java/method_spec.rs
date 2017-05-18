use super::_type::{AsType, Type};
use super::annotation_spec::AnnotationSpec;
use super::argument_spec::{AsArgumentSpec, ArgumentSpec};
use super::modifier::Modifiers;
use super::section::{AsSection, Sections};

#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub modifiers: Modifiers,
    pub name: String,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub returns: Option<Type>,
    pub sections: Sections,
}

impl MethodSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> MethodSpec {
        MethodSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            arguments: Vec::new(),
            returns: None,
            sections: Sections::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push_argument<A>(&mut self, argument: A)
        where A: AsArgumentSpec
    {
        self.arguments.push(argument.as_argument_spec());
    }

    pub fn returns<I>(&mut self, returns: I)
        where I: AsType
    {
        self.returns = Some(returns.as_type())
    }

    pub fn push<S>(&mut self, section: S)
        where S: AsSection
    {
        self.sections.push(section);
    }
}
