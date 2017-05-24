/// Module that adds fasterxml annotations to generated classes.
use parser::ast;
use super::processor;

use codeviz::java::*;
use errors::*;

pub struct Module {
    json_creator: ClassType,
    json_value: ClassType,
    json_property: ClassType,
    json_sub_types: ClassType,
    json_type_info: ClassType,
    illegal_argument: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module {
            json_creator: Type::class("com.fasterxml.jackson.annotation", "JsonCreator"),
            json_value: Type::class("com.fasterxml.jackson.annotation", "JsonValue"),
            json_property: Type::class("com.fasterxml.jackson.annotation", "JsonProperty"),
            json_sub_types: Type::class("com.fasterxml.jackson.annotation", "JsonSubTypes"),
            json_type_info: Type::class("com.fasterxml.jackson.annotation", "JsonTypeInfo"),
            illegal_argument: Type::class("java.lang", "IllegalArgumentException"),
        }
    }

    fn find_field(&self, fields: &Vec<processor::Field>, name: &str) -> Result<processor::Field> {
        for field in fields {
            if field.name == name {
                return Ok(field.clone());
            }
        }

        Err(format!("no field named: {}", name).into())
    }

    fn enum_from_value_method(&self,
                              field: &processor::Field,
                              class_type: &ClassType)
                              -> Result<MethodSpec> {
        let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &field.name);

        let value = java_stmt!["value"];

        let cond = match field.ty {
            Type::Primitive(_) => java_stmt!["this.", &field.name, " == ", &argument],
            _ => java_stmt!["this.", &field.name, ".equals(", &argument, ")"],
        };

        let mut return_matched = Elements::new();

        return_matched.push(java_stmt!["if (", &cond, ") {"]);
        return_matched.push_nested(java_stmt!["return ", &value, ";"]);
        return_matched.push("}");

        let mut value_loop = Elements::new();

        value_loop.push(java_stmt!["for (final ", class_type, " ", &value, " : this.values()) {"]);
        value_loop.push_nested(return_matched);
        value_loop.push("}");

        let mut from_value = MethodSpec::new(java_mods![Modifier::Public, Modifier::Static],
                                             "fromValue");

        let argument_name = Variable::String(argument.name.clone());
        let throw = java_stmt!["throw new ", &self.illegal_argument, "(", argument_name, ");"];

        from_value.returns(class_type);
        from_value.push_annotation(&self.json_creator);
        from_value.push_argument(argument);
        from_value.push(value_loop);
        from_value.push(throw);

        Ok(from_value)
    }

    fn enum_to_value_method(&self, field: &processor::Field) -> Result<MethodSpec> {
        let mut to_value = MethodSpec::new(java_mods![Modifier::Public], "toValue");

        to_value.returns(&field.ty);
        to_value.push_annotation(&self.json_value);
        to_value.push(java_stmt!["return this.", &field.name, ";"]);

        Ok(to_value)
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

    fn enum_added(&self,
                  enum_body: &ast::EnumBody,
                  fields: &Vec<processor::Field>,
                  class_type: &ClassType,
                  en: &mut EnumSpec)
                  -> Result<()> {
        if let Some(serialize_as) = enum_body.options.lookup_identifier_nth("serialize_as", 0) {
            let field = self.find_field(fields, serialize_as)?;

            let from_value = self.enum_from_value_method(&field, class_type)?;
            en.push(from_value);

            let to_value = self.enum_to_value_method(&field)?;
            en.push(to_value);
        }

        Ok(())
    }

    fn interface_added(&self,
                       interface: &ast::InterfaceBody,
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
                      _interface: &ast::InterfaceBody,
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
