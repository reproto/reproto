use backend::Backend;
use codegen::java::*;
use environment::Environment;
use options::Options;
use parser::ast;
use super::processor;

use errors::*;

pub struct FasterXmlBackend {
    json_creator: ClassType,
    json_property: ClassType,
    json_type_name: ClassType,
    json_sub_types: ClassType,
    json_type_info: ClassType,
}

impl FasterXmlBackend {
    pub fn new() -> FasterXmlBackend {
        FasterXmlBackend {
            json_creator: Type::class("com.fasterxml.jackson.annotation", "JsonCreator"),
            json_property: Type::class("com.fasterxml.jackson.annotation", "JsonProperty"),
            json_type_name: Type::class("com.fasterxml.jackson.annotation", "JsonTypeName"),
            json_sub_types: Type::class("com.fasterxml.jackson.annotation", "JsonSubTypes"),
            json_type_info: Type::class("com.fasterxml.jackson.annotation", "JsonTypeInfo"),
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
            let mut property = AnnotationSpec::new(&self.json_property);
            property.push_argument(&stmt![Variable::String(argument.name.clone())]);
            argument.push_annotation(&property);
        }

        Ok(())
    }

    fn interface_added(&self,
                       interface: &ast::InterfaceDecl,
                       interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        {
            let mut arguments = Statement::new();

            arguments.push(stmt!["use = ", &self.json_type_info, ".Id.NAME"]);
            arguments.push(stmt!["include = ", &self.json_type_info, ".As.PROPERTY"]);
            arguments.push(stmt!["property = ", Variable::String("type".to_owned())]);

            let mut type_info = AnnotationSpec::new(&self.json_type_info);
            type_info.push_argument(&stmt![arguments.join(", ")]);

            interface_spec.push_annotation(&type_info);
        }

        {
            let mut arguments = Statement::new();

            for (key, _sub_type) in &interface.sub_types {
                arguments.push(stmt!["@",
                                     &self.json_sub_types,
                                     ".Type(",
                                     &interface_spec.name,
                                     ".",
                                     key,
                                     ".class)"]);
            }

            let mut sub_types = AnnotationSpec::new(&self.json_sub_types);
            sub_types.push_argument(&stmt!["{", arguments.join(", "), "}"]);

            interface_spec.push_annotation(&sub_types);
        }

        Ok(())
    }

    fn sub_type_added(&self,
                      _interface: &ast::InterfaceDecl,
                      sub_type: &ast::SubType,
                      class: &mut ClassSpec)
                      -> Result<()> {

        if let Some(name) = sub_type.options.lookup_string_nth("name", 0) {
            let mut type_name = AnnotationSpec::new(&self.json_type_name);
            type_name.push_argument(&stmt![Variable::String(name.clone())]);
            class.push_annotation(&type_name);
        }

        Ok(())
    }
}

impl Backend for FasterXmlBackend {
    fn process(&self, options: &Options, env: &Environment) -> Result<()> {
        let package_prefix = options.package_prefix
            .clone()
            .map(|prefix| ast::Package::new(prefix.split(".").map(ToOwned::to_owned).collect()));

        let processor = processor::Processor::new(options, env, package_prefix);
        processor.process(self)
    }
}
