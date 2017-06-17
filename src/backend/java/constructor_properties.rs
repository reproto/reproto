/// Module that adds fasterxml annotations to generated classes.
use backend::*;
use codeviz::java::*;
use super::processor::*;

pub struct Module {
    constructor_properties: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module { constructor_properties: Type::class("java.beans", "ConstructorProperties") }
    }
}

impl Listeners for Module {
    fn class_added(&self, event: &mut ClassAdded) -> Result<()> {
        if event.spec.constructors.len() != 1 {
            return Err("Expected exactly one constructor".into());
        }

        let constructor = &mut event.spec.constructors[0];
        let mut arguments = Statement::new();

        for field in event.fields {
            arguments.push(stmt![Variable::String(field.name.to_owned())]);
        }

        let mut annotation = AnnotationSpec::new(&self.constructor_properties);
        annotation.push_argument(stmt!["{", arguments.join(", "), "}"]);
        constructor.push_annotation(&annotation);

        Ok(())
    }
}
