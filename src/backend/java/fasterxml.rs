/// Module that adds fasterxml annotations to generated classes.
use parser::ast;
use super::processor;

use codegen::java::*;
use errors::*;

pub struct Module {
    json_creator: ClassType,
    json_property: ClassType,
    json_sub_types: ClassType,
    json_type_info: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module {
            json_creator: Type::class("com.fasterxml.jackson.annotation", "JsonCreator"),
            json_property: Type::class("com.fasterxml.jackson.annotation", "JsonProperty"),
            json_sub_types: Type::class("com.fasterxml.jackson.annotation", "JsonSubTypes"),
            json_type_info: Type::class("com.fasterxml.jackson.annotation", "JsonTypeInfo"),
        }
    }
}

impl processor::Listeners for Module {
    fn class_added(&self,
                   fields: &Vec<processor::Field>,
                   _class_type: &ClassType,
                   class: &mut ClassSpec)
                   -> Result<()> {
        if class.constructors.len() != 1 {
            return Err("Expected exactly one constructor".into());
        }

        let constructor = &mut class.constructors[0];
        let creator_annotation = AnnotationSpec::new(&self.json_creator);

        constructor.push_annotation(&creator_annotation);

        let mut field_it = fields.iter();

        for argument in &mut constructor.arguments {
            if let Some(field) = field_it.next() {
                let mut property = AnnotationSpec::new(&self.json_property);
                property.push_argument(java_stmt![Variable::String(field.name.clone())]);
                argument.push_annotation(&property);
            } else {
                return Err("missing field".into());
            }
        }

        Ok(())
    }

    fn interface_added(&self,
                       interface: &ast::TypeBody,
                       interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        {
            let mut arguments = Statement::new();

            arguments.push(java_stmt!["use=", &self.json_type_info, ".Id.NAME"]);
            arguments.push(java_stmt!["include=", &self.json_type_info, ".As.PROPERTY"]);
            arguments.push(java_stmt!["property=", Variable::String("type".to_owned())]);

            let mut type_info = AnnotationSpec::new(&self.json_type_info);
            type_info.push_argument(arguments.join(", "));

            interface_spec.push_annotation(&type_info);
        }

        {
            let mut arguments = Statement::new();

            for (key, sub_type) in &interface.sub_types {
                for name in &sub_type.options.lookup_string("name") {
                    let name: String = (*name).clone();

                    let mut type_args = Statement::new();

                    type_args.push(java_stmt!["name=", Variable::String(name)]);
                    type_args.push(java_stmt!["value=", &interface_spec.name, ".", key, ".class"]);

                    let a =
                        java_stmt!["@", &self.json_sub_types, ".Type(", type_args.join(", "), ")"];

                    arguments.push(a);
                }
            }

            let mut sub_types = AnnotationSpec::new(&self.json_sub_types);
            sub_types.push_argument(java_stmt!["{", arguments.join(", "), "}"]);

            interface_spec.push_annotation(&sub_types);
        }

        Ok(())
    }

    fn sub_type_added(&self,
                      _fields: &Vec<processor::Field>,
                      _interface: &ast::TypeBody,
                      _sub_type: &ast::SubType,
                      _class: &mut ClassSpec)
                      -> Result<()> {
        // if let Some(name) = sub_type.options.lookup_string_nth("name", 0) {
        // let mut type_name = AnnotationSpec::new(&self.json_type_name);
        // type_name.push_argument(java_stmt![Variable::String(name.clone())]);
        // class.push_annotation(&type_name);
        // }

        Ok(())
    }
}
