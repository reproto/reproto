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
    serialize: ClassType,
    deserialize: ClassType,
    deserializer: ClassType,
    serializer: ClassType,
    generator: ClassType,
    serializer_provider: ClassType,
    parser: ClassType,
    deserialization_context: ClassType,
    token: ClassType,
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
            serialize: Type::class("com.fasterxml.jackson.annotation", "JsonSerialize"),
            deserialize: Type::class("com.fasterxml.jackson.annotation", "JsonDeserialize"),
            serializer: Type::class("com.fasterxml.jackson.databind", "JsonSerializer"),
            deserializer: Type::class("com.fasterxml.jackson.databind", "JsonDeserializer"),
            generator: Type::class("com.fasterxml.jackson.core", "JsonGenerator"),
            serializer_provider: Type::class("com.fasterxml.jackson.databind",
                                             "JsonSerializerProvider"),
            parser: Type::class("com.fasterxml.jackson.core", "JsonParser"),
            deserialization_context: Type::class("com.fasterxml.jackson.databind",
                                                 "DeserializationContext"),
            token: Type::class("com.fasterxml.jackson.core", "JsonToken"),
            string: Type::class("java.lang", "String"),
        }
    }

    /// Custom serialize implementation for tuples.
    fn tuple_serializer(&self,
                        fields: &Vec<processor::Field>,
                        class_type: &ClassType)
                        -> Result<ClassSpec> {
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

        Ok(serializer)
    }

    fn deserialize_method_for_type(&self,
                                   ty: &Type,
                                   parser: &ArgumentSpec)
                                   -> Result<(Option<&str>, Statement)> {
        match *ty {
            Type::Primitive(ref primitive) => {
                match *primitive {
                    SHORT => Ok((Some("NUMBER"), java_stmt![parser, ".getShortValue()"])),
                    LONG => Ok((Some("NUMBER"), java_stmt![parser, ".getLongValue()"])),
                    INTEGER => Ok((Some("NUMBER"), java_stmt![parser, ".getIntegerValue()"])),
                    FLOAT => Ok((Some("NUMBER"), java_stmt![parser, ".getFloatValue()"])),
                    DOUBLE => Ok((Some("NUMBER"), java_stmt![parser, ".getDoubleValue()"])),
                    _ => return Err("cannot deserialize type".into()),
                }
            }
            Type::Class(ref class) => {
                if *class == self.string {
                    return Ok((Some("STRING"), java_stmt![parser, ".getText()"]));
                }

                if class.arguments.is_empty() {
                    return Ok((None, java_stmt![parser, ".readValueAs(", class, ".class)"]));
                }

                // TODO: support generics
                return Err("cannot deserialize type".into());
            }
            Type::Local(ref local) => {
                return Ok((None, java_stmt![parser, ".readValueAs(", &local.name, ")"]));
            }
        }
    }

    fn wrong_token_exception(&self,
                             ctxt: &ArgumentSpec,
                             parser: &ArgumentSpec,
                             token: &str)
                             -> Statement {
        java_stmt!["throw ",
                   ctxt,
                   ".wrongTokenException(",
                   parser,
                   ", ",
                   &self.token,
                   ",",
                   token,
                   ", null);"]
    }

    /// Custom deserialize implementation for tuples.
    fn tuple_deserializer(&self,
                          fields: &Vec<processor::Field>,
                          class_type: &ClassType)
                          -> Result<ClassSpec> {
        let mut deserializer = ClassSpec::new(java_mods![Modifier::Public, Modifier::Static],
                                              "deserializer");

        deserializer.extends(self.deserializer.with_arguments(vec![&class_type]));

        let parser = ArgumentSpec::new(java_mods![Modifier::Final], &self.parser, "parser");
        let ctxt = ArgumentSpec::new(java_mods![Modifier::Final],
                                     &self.deserialization_context,
                                     "ctxt");

        let mut deserialize = MethodSpec::new(java_mods![Modifier::Public], "deserialize");
        deserialize.push_argument(&parser);
        deserialize.push_argument(&ctxt);
        deserialize.push_annotation(&self.override_);
        deserialize.returns(&class_type);

        let next_token = java_stmt![&parser, ".nextToken()"];

        let mut start_array = Elements::new();
        start_array.push(java_stmt!["if (", &next_token, " != ", &self.token, ".START_ARRAY) {"]);
        start_array.push_nested(self.wrong_token_exception(&ctxt, &parser, "START_ARRAY"));
        start_array.push("}");
        deserialize.push(start_array);

        let mut arguments = Statement::new();

        for field in fields {
            let (token, reader) = self.deserialize_method_for_type(&field.ty, &parser)?;

            if let Some(token) = token {
                let mut field_check = Elements::new();
                field_check.push(java_stmt!["if (", &next_token, " != ", &self.token, ".", token, ") {"]);
                field_check.push_nested(self.wrong_token_exception(&ctxt, &parser, token));
                field_check.push("}");
                deserialize.push(field_check);
            }

            let variable = java_stmt!["v_", &field.field_spec.name];

            deserialize.push(java_stmt!["final ", &variable, " = ", reader, ";"]);
            arguments.push(variable);
        }

        let mut end_array = Elements::new();
        end_array.push(java_stmt!["if (", &next_token, " != ", &self.token, ".END_ARRAY) {"]);
        end_array.push_nested(self.wrong_token_exception(&ctxt, &parser, "END_ARRAY"));
        end_array.push("}");
        deserialize.push(end_array);

        deserialize.push(java_stmt!["return new ", &class_type, "(", arguments.join(", "), ");"]);

        deserializer.push(deserialize);
        Ok(deserializer)
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

        let serializer = self.tuple_serializer(fields, class_type)?;

        let serializer_type = Type::class(&class_type.package,
                                          &format!("{}.{}", class_type.name, serializer.name));

        let mut serialize_annotation: AnnotationSpec = self.serialize.clone().into();
        serialize_annotation.push_argument(java_stmt!["using = ", serializer_type, ".class"]);

        class.push_annotation(serialize_annotation);
        class.push(serializer);

        let deserializer = self.tuple_deserializer(fields, class_type)?;

        let deserializer_type = Type::class(&class_type.package,
                                            &format!("{}.{}", class_type.name, deserializer.name));

        let mut deserialize_annotation: AnnotationSpec = self.deserialize.clone().into();
        deserialize_annotation.push_argument(java_stmt!["using = ", deserializer_type, ".class"]);

        class.push_annotation(deserialize_annotation);
        class.push(deserializer);

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
