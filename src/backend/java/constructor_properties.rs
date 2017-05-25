/// Module that adds fasterxml annotations to generated classes.
use super::processor;

use codeviz::java::*;
use backend::*;

pub struct Module {
    constructor_properties: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module { constructor_properties: Type::class("java.beans", "ConstructorProperties") }
    }
}

impl Module {
    fn add_constructor_properties(&self,
                                  fields: &Vec<processor::Field>,
                                  class: &mut ClassSpec)
                                  -> Result<()> {
        if class.constructors.len() != 1 {
            return Err("Expected exactly one constructor".into());
        }

        let constructor = &mut class.constructors[0];
        let mut arguments = Statement::new();

        for field in fields {
            arguments.push(java_stmt![Variable::String(field.name.clone())]);
        }

        let mut annotation = AnnotationSpec::new(&self.constructor_properties);
        annotation.push_argument(java_stmt!["{", arguments.join(", "), "}"]);
        constructor.push_annotation(&annotation);

        Ok(())
    }
}

impl processor::Listeners for Module {
    fn class_added(&self,
                   fields: &Vec<processor::Field>,
                   _class_type: &ClassType,
                   class: &mut ClassSpec)
                   -> Result<()> {
        self.add_constructor_properties(fields, class)?;
        Ok(())
    }
}
