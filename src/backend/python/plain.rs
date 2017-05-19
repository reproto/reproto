use backend::Backend;
use codegen::python::*;
use environment::Environment;
use options::Options;
use parser::ast;
use super::processor;

use errors::*;

pub struct PlainPythonBackend {
    staticmethod: BuiltInName,
    dict: BuiltInName,
}

impl PlainPythonBackend {
    pub fn new() -> PlainPythonBackend {
        PlainPythonBackend {
            staticmethod: Name::built_in("staticmethod"),
            dict: Name::built_in("dict"),
        }
    }

    fn raise_if_none(&self, stmt: &Statement, field: &processor::Field) -> Elements {
        let mut raise_if_none = Elements::new();
        let required_error = Variable::String(format!("{}: is a required field", field.name));

        raise_if_none.push(python_stmt!["if ", &stmt, " is None:"]);
        raise_if_none.push_nested(python_stmt!["raise Exception(", required_error, ")"]);

        raise_if_none
    }

    fn encode_method<F>(&self,
                        processor: &processor::Processor,
                        package: &ast::Package,
                        fields: &Vec<processor::Field>,
                        builder: &BuiltInName,
                        field_set: F)
                        -> Result<MethodSpec>
        where F: Fn(&str, Statement) -> Statement
    {
        let mut encode = MethodSpec::new("encode");
        encode.push_argument(python_stmt!["self"]);

        let mut encode_body = Elements::new();

        encode_body.push(python_stmt!["data = ", builder, "()"]);

        for field in fields {
            match field.modifier {
                ast::Modifier::Optional => {
                    let mut check_if_none = Elements::new();
                    let stmt = python_stmt!["self.", &field.ident];

                    check_if_none.push(python_stmt!["if ", &stmt, " is not None:"]);

                    let stmt = processor.encode(package, &field.ty, stmt.clone())?;
                    let stmt = field_set(&field.name, stmt);

                    check_if_none.push_nested(stmt);

                    encode_body.push(check_if_none);
                }
                _ => {
                    let stmt = python_stmt!["self.", &field.ident];

                    // TODO: make configurable
                    encode_body.push(self.raise_if_none(&stmt, field));

                    let stmt = python_stmt!["self.", &field.name];
                    let stmt = processor.encode(package, &field.ty, stmt)?;
                    let stmt = field_set(&field.name, stmt);

                    encode_body.push(stmt);
                }
            }
        }

        encode_body.push(python_stmt!["return data"]);

        encode.push(encode_body.join(ElementSpec::Spacing));
        Ok(encode)
    }

    fn encode_tuple_method(&self,
                           processor: &processor::Processor,
                           package: &ast::Package,
                           fields: &Vec<processor::Field>)
                           -> Result<MethodSpec> {
        let mut values = Statement::new();

        let mut encode = MethodSpec::new("encode");
        encode.push_argument(python_stmt!["self"]);

        let mut encode_body = Elements::new();

        for field in fields {
            let stmt = python_stmt!["self.", &field.name];
            encode_body.push(self.raise_if_none(&stmt, field));
            values.push(processor.encode(package, &field.ty, stmt)?);
        }

        encode_body.push(python_stmt!["return (", values.join(", "), ")"]);
        encode.push(encode_body.join(ElementSpec::Spacing));
        Ok(encode)
    }

    fn optional_check(&self, var_name: &str, index: &Variable, stmt: &Statement) -> ElementSpec {
        let mut check = Elements::new();

        let mut none_check = Elements::new();
        none_check.push(python_stmt![var_name, " = data[", index, "]"]);

        let mut none_check_if = Elements::new();

        let assign_var = python_stmt![var_name, " = ", stmt];

        none_check_if.push(python_stmt!["if ", var_name, " is not None:"]);
        none_check_if.push_nested(assign_var);

        none_check.push(none_check_if);

        check.push(python_stmt!["if ", index, " in data:"]);
        check.push_nested(none_check.join(ElementSpec::Spacing));

        check.push(python_stmt!["else:"]);
        check.push_nested(python_stmt![var_name, " = None"]);

        check.as_element_spec()
    }

    fn decode_method<F>(&self,
                        processor: &processor::Processor,
                        package: &ast::Package,
                        fields: &Vec<processor::Field>,
                        class: &ClassSpec,
                        variable_fn: F)
                        -> Result<MethodSpec>
        where F: Fn(usize, &str) -> Variable
    {
        let mut decode = MethodSpec::new("decode");
        decode.push_decorator(&self.staticmethod);
        decode.push_argument(python_stmt!["data"]);

        let mut decode_body = Elements::new();

        let mut arguments = Statement::new();

        for (i, field) in fields.iter().enumerate() {
            let var_name = format!("f_{}", field.ident);
            let var = variable_fn(i, &field.name);

            let stmt = match field.modifier {
                ast::Modifier::Optional => {
                    let var_stmt = python_stmt![&var_name];
                    let var_stmt = processor.decode(package, &field.ty, var_stmt)?;

                    self.optional_check(&var_name, &var, &var_stmt)
                }
                _ => {
                    let var_stmt = python_stmt!["data[", &var, "]"];
                    let var_stmt = processor.decode(package, &field.ty, var_stmt)?;

                    python_stmt![&var_name, " = ", &var_stmt].as_element_spec()
                }
            };

            decode_body.push(stmt);
            arguments.push(var_name);
        }

        let arguments = arguments.join(", ");
        decode_body.push(python_stmt!["return ", &class.name, "(", arguments, ")"]);

        decode.push(decode_body.join(ElementSpec::Spacing));

        Ok(decode)
    }
}

impl processor::Listeners for PlainPythonBackend {
    fn class_added(&self,
                   processor: &processor::Processor,
                   package: &ast::Package,
                   fields: &Vec<processor::Field>,
                   class: &mut ClassSpec)
                   -> Result<()> {

        let decode = self.decode_method(processor,
                           package,
                           fields,
                           class,
                           |_, name| Variable::String(name.to_owned()))?;

        let encode = self.encode_method(processor, package, fields, &self.dict, |name, stmt| {
                python_stmt!["data[", Variable::String(name.to_owned()), "] = ", stmt]
            })?;

        class.push(decode);
        class.push(encode);
        Ok(())
    }

    fn tuple_added(&self,
                   processor: &processor::Processor,
                   package: &ast::Package,
                   fields: &Vec<processor::Field>,
                   class: &mut ClassSpec)
                   -> Result<()> {

        let decode = self.decode_method(processor,
                           package,
                           fields,
                           class,
                           |i, _| Variable::Literal(i.to_string()))?;

        let encode = self.encode_tuple_method(processor, package, fields)?;

        class.push(decode);
        class.push(encode);
        Ok(())
    }

    fn interface_added(&self,
                       _processor: &processor::Processor,
                       _package: &ast::Package,
                       interface: &ast::InterfaceDecl,
                       interface_spec: &mut ClassSpec)
                       -> Result<()> {
        let mut decode = MethodSpec::new("decode");
        decode.push_decorator(&self.staticmethod);
        decode.push_argument(python_stmt!["data"]);

        let mut decode_body = Elements::new();

        decode_body.push(python_stmt!["type = data[", Variable::String("type".to_owned()), "]"]);

        for (_, ref sub_type) in &interface.sub_types {
            for name in sub_type.options.lookup_string("name") {
                let type_name = Name::local(&sub_type.name).as_name();

                let mut check = Elements::new();

                check.push(python_stmt!["if type == ", Variable::String(name.to_owned()), ":"]);
                check.push_nested(python_stmt!["return ", type_name, ".decode(data)"]);

                decode_body.push(check);
            }
        }

        decode_body.push(python_stmt!["raise Exception(",
                                      Variable::String("bad type".to_owned()),
                                      " + type)"]);

        decode.push(decode_body.join(ElementSpec::Spacing));
        interface_spec.push(decode);

        Ok(())
    }
}

impl Backend for PlainPythonBackend {
    fn process(&self, options: &Options, env: &Environment) -> Result<()> {
        let package_prefix = options.package_prefix
            .clone()
            .map(|prefix| ast::Package::new(prefix.split(".").map(ToOwned::to_owned).collect()));

        let processor = processor::Processor::new(options, env, package_prefix);
        processor.process(self)
    }
}
