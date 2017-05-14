use super::processor;

use backend::Backend;
use options::Options;
use parser::ast;
use codegen::java::*;

use errors::*;

pub struct FasterXmlBackend {
    processor: processor::Processor,
    json_creator: Type,
    json_property: Type,
}

impl FasterXmlBackend {
    pub fn new() -> FasterXmlBackend {
        FasterXmlBackend {
            processor: processor::Processor::new(),
            json_creator: Type::new("com.fasterxml.jackson.annotation", "JsonCreator"),
            json_property: Type::new("com.fasterxml.jackson.annotation", "JsonProperty"),
        }
    }
}

impl processor::Listeners for FasterXmlBackend {
    fn class_added(&self, class: &mut ClassSpec) -> Result<()> {
        if class.constructors.len() != 1 {
            return Err("Expected exactly one constructor".into());
        }

        let constructor = &mut class.constructors[0];
        let creator_annotation = AnnotationSpec::new(&self.json_creator);

        constructor.push_annotation(&creator_annotation);

        for argument in &mut constructor.arguments {
            let mut property_annotation = AnnotationSpec::new(&self.json_property);
            property_annotation.push_argument(&stmt!["$S", string argument.name]);
            argument.push_annotation(&property_annotation);
        }

        Ok(())
    }
}

impl Backend for FasterXmlBackend {
    fn add_file(&mut self, file: ast::File) -> Result<()> {
        self.processor.add_file(file)
    }

    fn process(&self, options: &Options) -> Result<()> {
        self.processor.process(options, self)
    }
}
