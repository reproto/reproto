use super::annotation_spec::{AsAnnotationSpec, AnnotationSpec};
use super::argument_spec::{AsArgumentSpec, ArgumentSpec};
use super::block::Block;
use super::modifier::Modifiers;
use super::section::{AsSection, Sections};
use super::statement::{AsStatement, Statement};
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct ConstructorSpec {
    pub modifiers: Modifiers,
    pub annotations: Vec<AnnotationSpec>,
    pub arguments: Vec<ArgumentSpec>,
    pub sections: Sections,
}

impl ConstructorSpec {
    pub fn new(modifiers: Modifiers) -> ConstructorSpec {
        ConstructorSpec {
            modifiers: modifiers,
            annotations: Vec::new(),
            arguments: Vec::new(),
            sections: Sections::new(),
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

    pub fn push<S>(&mut self, section: S)
        where S: AsSection
    {
        self.sections.push(section);
    }

    pub fn as_block(&self, enclosing: &str) -> Block {
        let mut open = Statement::new();

        for a in &self.annotations {
            open.push(a.as_statement());
            open.push(Variable::Spacing);
        }

        if !self.modifiers.is_empty() {
            open.push(&self.modifiers);
            open.push(" ");
        }

        open.push(enclosing);
        open.push("(");
        open.push_arguments(&self.arguments, ", ");
        open.push(")");

        let mut block = Block::new();
        block.open(open);
        block.extend(&self.sections);

        block
    }
}
