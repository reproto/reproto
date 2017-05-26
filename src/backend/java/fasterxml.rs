/// Module that adds fasterxml annotations to generated classes.
use backend::*;
use backend::models as m;
use codeviz::java::*;
use super::processor;

pub struct Module {
    override_: ClassType,
    creator: ClassType,
    value: ClassType,
    property: ClassType,
    sub_types: ClassType,
    type_info: ClassType,
    serializer: ClassType,
    generator: ClassType,
    serializer_provider: ClassType,
    string: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module {
            override_: Type::class("java.lang", "Override"),
            creator: Type::class("com.fasterxml.jackson.annotation", "JsonCreator"),
            value: Type::class("com.fasterxml.jackson.annotation", "JsonValue"),
            property: Type::class("com.fasterxml.jackson.annotation", "JsonProperty"),
            sub_types: Type::class("com.fasterxml.jackson.annotation", "JsonSubTypes"),
            type_info: Type::class("com.fasterxml.jackson.annotation", "JsonTypeInfo"),
            serializer: Type::class("com.fasterxml.jackson.databind", "JsonSerializer"),
            generator: Type::class("com.fasterxml.jackson.databind", "JsonGenerator"),
            serializer_provider: Type::class("com.fasterxml.jackson.databind",
                                             "JsonSerializerProvider"),
            string: Type::class("java.lang", "String"),
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
        let creator_annotation = AnnotationSpec::new(&self.creator);

        constructor.push_annotation(&creator_annotation);

        if constructor.arguments.len() != fields.len() {
            return Err(format!("The number of constructor arguments ({}) did not match the \
                                number of fields ({})",
                               constructor.arguments.len(),
                               fields.len())
                .into());
        }

        let zipped = constructor.arguments.iter_mut().zip(fields.iter());

        for (argument, field) in zipped {
            let mut property = AnnotationSpec::new(&self.property);
            property.push_argument(java_stmt![Variable::String(field.name.clone())]);
            argument.push_annotation(&property);
        }

        Ok(())
    }

    fn tuple_added(&self,
                   fields: &Vec<processor::Field>,
                   class_type: &ClassType,
                   class: &mut ClassSpec)
                   -> Result<()> {
        let mut serializer = ClassSpec::new(java_mods![Modifier::Public, Modifier::Static],
                                            "Serializer");

        serializer.extends(self.serializer.with_arguments(vec![&class_type]));

        let value = ArgumentSpec::new(java_mods![Modifier::Final], &class_type, "value");
        let jgen = ArgumentSpec::new(java_mods![Modifier::Final], &self.generator, "jgen");
        let provider = ArgumentSpec::new(java_mods![Modifier::Final],
                                         &self.serializer_provider,
                                         "provider");

        let mut serialize = MethodSpec::new(java_mods![Modifier::Public], "serialize");
        serialize.push_argument(&value);
        serialize.push_argument(&jgen);
        serialize.push_argument(&provider);
        serialize.push_annotation(&self.override_);

        let mut body = Elements::new();
        body.push(java_stmt![&jgen, ".writeStartArray();"]);

        for field in fields {
            let field_stmt = java_stmt!["this.", &field.field_spec];

            let write = match field.ty {
                Type::Primitive(ref primitive) => {
                    match *primitive {
                        SHORT | LONG | INTEGER | FLOAT | DOUBLE => {
                            java_stmt!["writeNumber(", field_stmt, ")"]
                        }
                        _ => return Err("cannot serialize type".into()),
                    }
                }
                Type::Class(ref class) => {
                    if *class == self.string {
                        java_stmt!["writeString(", field_stmt, ")"]
                    } else {
                        java_stmt!["writeObject(", field_stmt, ")"]
                    }
                }
                _ => java_stmt!["writeObject(", field_stmt, ")"],
            };

            body.push(java_stmt![&jgen, ".", write, ";"]);
        }

        body.push(java_stmt![&jgen, ".writeEndArray();"]);

        serialize.push(body);

        serializer.push(serialize);

        class.push(serializer);

        Ok(())
    }

    fn enum_added(&self,
                  _enum_body: &m::EnumBody,
                  _fields: &Vec<processor::Field>,
                  _class_type: &ClassType,
                  from_value: &mut Option<MethodSpec>,
                  to_value: &mut Option<MethodSpec>,
                  _en: &mut EnumSpec)
                  -> Result<()> {

        if let Some(ref mut from_value) = *from_value {
            from_value.push_annotation(&self.creator);
        }

        if let Some(ref mut to_value) = *to_value {
            to_value.push_annotation(&self.value);
        }

        Ok(())
    }

    fn interface_added(&self,
                       interface: &m::InterfaceBody,
                       interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        {
            let mut arguments = Statement::new();

            arguments.push(java_stmt!["use=", &self.type_info, ".Id.NAME"]);
            arguments.push(java_stmt!["include=", &self.type_info, ".As.PROPERTY"]);
            arguments.push(java_stmt!["property=", Variable::String("type".to_owned())]);

            let mut type_info = AnnotationSpec::new(&self.type_info);
            type_info.push_argument(arguments.join(", "));

            interface_spec.push_annotation(&type_info);
        }

        {
            let mut arguments = Statement::new();

            for (key, sub_type) in &interface.sub_types {
                for name in &sub_type.names {
                    let name: String = name.inner.to_owned();

                    let mut type_args = Statement::new();

                    type_args.push(java_stmt!["name=", Variable::String(name)]);
                    type_args.push(java_stmt!["value=", &interface_spec.name, ".", key, ".class"]);

                    let a = java_stmt!["@", &self.sub_types, ".Type(", type_args.join(", "), ")"];

                    arguments.push(a);
                }
            }

            let mut sub_types = AnnotationSpec::new(&self.sub_types);
            sub_types.push_argument(java_stmt!["{", arguments.join(", "), "}"]);

            interface_spec.push_annotation(&sub_types);
        }

        Ok(())
    }

    fn sub_type_added(&self,
                      _fields: &Vec<processor::Field>,
                      _interface: &m::InterfaceBody,
                      _sub_type: &m::SubType,
                      _class: &mut ClassSpec)
                      -> Result<()> {
        // if let Some(name) = sub_type.options.lookup_string_nth("name", 0) {
        // let mut type_name = AnnotationSpec::new(&self.type_name);
        // type_name.push_argument(java_stmt![Variable::String(name.clone())]);
        // class.push_annotation(&type_name);
        // }

        Ok(())
    }
}
