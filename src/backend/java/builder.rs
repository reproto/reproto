/// Module that adds fasterxml annotations to generated classes.
use backend::*;
use backend::models as m;
use codeviz::java::*;
use super::processor;

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
        let field_mods = mods![Modifier::Private];

        let ty = match field.modifier {
            m::Modifier::Required => self.optional.with_arguments(vec![&source.ty]).into(),
            _ => source.ty.clone(),
        };

        let mut spec = FieldSpec::new(field_mods, ty, &source.name);
        spec.initialize(stmt![&self.optional, ".empty()"]);
        spec
    }

    fn setter_method(&self, field: &processor::Field, source: &FieldSpec) -> MethodSpec {
        let mut setter = MethodSpec::new(mods![Modifier::Public], &source.name);

        let argument = ArgumentSpec::new(mods![Modifier::Final], &field.ty, &source.name);

        let value = stmt![&self.optional, ".of(", &argument, ")"];

        let mut setter_body = Elements::new();

        /// Use separate element to get nice spacing
        setter_body.push(stmt!["this.", &source.name, " = ", value, ";"]);
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
        let mut builder = ClassSpec::new(mods![Modifier::Public, Modifier::Static], "Builder");

        let mut build_variable_assign = Elements::new();
        let mut build_constructor_arguments = Statement::new();

        for field in fields {
            let source = &field.spec;

            builder.push_field(self.builder_field(field, source));
            builder.push(self.setter_method(field, source));

            let value = match field.modifier {
                m::Modifier::Required => {
                    let message = Variable::String(format!("{}: is required", source.name));
                    let throw_stmt = stmt!["new ", &self.runtime_exception, "(", message, ")"];

                    stmt!["this.", &source.name, ".orElseThrow(() -> ", throw_stmt, ")"]
                }
                _ => stmt!["this.", &source.name],
            };

            let assign = stmt!["final ", &source.ty, " ", &source.name, " = ", value, ";"];
            build_variable_assign.push(assign);
            build_constructor_arguments.push(&source.name);
        }

        let mut build = MethodSpec::new(mods![Modifier::Public], "build");
        build.returns(class_type);
        build.push(build_variable_assign);
        build.push(stmt!["return new ",
                         class_type,
                         "(",
                         build_constructor_arguments.join(", "),
                         ");"]);

        builder.push(build);

        class.push(builder);

        Ok(())
    }
}
