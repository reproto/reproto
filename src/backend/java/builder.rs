/// Module that adds fasterxml annotations to generated classes.
use super::processor;

use parser::ast;

use codegen::java::*;
use errors::*;

pub struct Module {
    optional: ClassType,
    runtime_exception: ClassType,
}

impl Module {
    pub fn new() -> Module {
        Module {
            optional: Type::class("java.util", "Optional"),
            runtime_exception: Type::class("java.lang", "RuntimeException"),
        }
    }
}

impl Module {
    fn builder_field(&self, field: &processor::Field, source: &FieldSpec) -> FieldSpec {
        let field_mods = java_mods![Modifier::Private];

        let ty = match field.modifier {
            ast::Modifier::Required => self.optional.with_arguments(vec![&source.ty]).as_type(),
            _ => source.ty.clone(),
        };

        let mut field_spec = FieldSpec::new(field_mods, ty, &source.name);
        field_spec.initialize(java_stmt![&self.optional, ".empty()"]);
        field_spec
    }

    fn setter_method(&self, field: &processor::Field, source: &FieldSpec) -> MethodSpec {
        let mut setter = MethodSpec::new(java_mods![Modifier::Public], &source.name);

        let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &source.name);

        let value = java_stmt![&self.optional, ".of(", &argument, ")"];

        let mut setter_body = Elements::new();

        /// Use separate element to get nice spacing
        setter_body.push(java_stmt!["this.", &source.name, " = ", value, ";"]);
        setter_body.push("return this;");

        setter.push(setter_body);
        setter.returns(Type::local("Builder"));
        setter.push_argument(argument);

        setter
    }
}

impl processor::Listeners for Module {
    fn class_added(&self,
                   fields: &Vec<processor::Field>,
                   class_type: &ClassType,
                   class: &mut ClassSpec)
                   -> Result<()> {
        let mut builder = ClassSpec::new(java_mods![Modifier::Public, Modifier::Static], "Builder");

        let mut build_variable_assign = Elements::new();
        let mut build_constructor_arguments = Statement::new();

        for field in fields {
            let source = &field.field_spec;

            builder.push_field(self.builder_field(field, source));
            builder.push(self.setter_method(field, source));

            let value = match field.modifier {
                ast::Modifier::Required => {
                    let message = Variable::String(format!("{}: is required", source.name));
                    let throw_stmt = java_stmt!["new ", &self.runtime_exception, "(", message, ")"];

                    java_stmt!["this.", &source.name, ".orElseThrow(() -> ", throw_stmt, ")"]
                }
                _ => java_stmt!["this.", &source.name],
            };

            build_variable_assign.push(java_stmt!["final ", &source.ty, " ", &source.name, " = ", value, ";"]);
            build_constructor_arguments.push(&source.name);
        }

        let mut build = MethodSpec::new(java_mods![Modifier::Public], "build");
        build.returns(class_type);
        build.push(build_variable_assign);
        build.push(java_stmt!["return new ",
                              class_type,
                              "(",
                              build_constructor_arguments.join(", "),
                              ");"]);

        builder.push(build);

        class.push(builder);

        Ok(())
    }
}
