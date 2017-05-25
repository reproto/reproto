use backend::Backend;
use environment::Environment;
use naming::{self, FromNaming};
use options::Options;
use parser::ast;
use std::fs::File;
use std::fs;
use std::io::Write;

use codeviz::java::*;

use errors::*;

const JAVA_CONTEXT: &str = "java";

pub trait Listeners {
    fn configure(&self, _options: &mut ProcessorOptions) -> Result<()> {
        Ok(())
    }

    fn class_added(&self,
                   _fields: &Vec<Field>,
                   _class_type: &ClassType,
                   _class: &mut ClassSpec)
                   -> Result<()> {
        Ok(())
    }

    fn enum_added(&self,
                  _enum_body: &ast::EnumBody,
                  _fields: &Vec<Field>,
                  _class_type: &ClassType,
                  _from_value: &mut Option<MethodSpec>,
                  _to_value: &mut Option<MethodSpec>,
                  _class: &mut EnumSpec)
                  -> Result<()> {
        Ok(())
    }

    fn interface_added(&self,
                       _interface: &ast::InterfaceBody,
                       _interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        Ok(())
    }

    fn sub_type_added(&self,
                      _fields: &Vec<Field>,
                      _interface: &ast::InterfaceBody,
                      _sub_type: &ast::SubType,
                      _class: &mut ClassSpec)
                      -> Result<()> {
        Ok(())
    }
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    fn configure(&self, processor: &mut ProcessorOptions) -> Result<()> {
        for listeners in self {
            listeners.configure(processor)?;
        }

        Ok(())
    }

    fn class_added(&self,
                   fields: &Vec<Field>,
                   class_type: &ClassType,
                   class: &mut ClassSpec)
                   -> Result<()> {
        for listeners in self {
            listeners.class_added(fields, class_type, class)?;
        }

        Ok(())
    }

    fn enum_added(&self,
                  enum_body: &ast::EnumBody,
                  fields: &Vec<Field>,
                  class_type: &ClassType,
                  from_value: &mut Option<MethodSpec>,
                  to_value: &mut Option<MethodSpec>,
                  class: &mut EnumSpec)
                  -> Result<()> {
        for listeners in self {
            listeners.enum_added(enum_body, fields, class_type, from_value, to_value, class)?;
        }

        Ok(())
    }

    fn interface_added(&self,
                       interface: &ast::InterfaceBody,
                       interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        for listeners in self {
            listeners.interface_added(interface, interface_spec)?;
        }

        Ok(())
    }

    fn sub_type_added(&self,
                      fields: &Vec<Field>,
                      interface: &ast::InterfaceBody,
                      sub_type: &ast::SubType,
                      class: &mut ClassSpec)
                      -> Result<()> {
        for listeners in self {
            listeners.sub_type_added(fields, interface, sub_type, class)?;
        }

        Ok(())
    }
}

/// A single field.
#[derive(Debug, Clone)]
pub struct Field {
    pub modifier: ast::Modifier,
    pub name: String,
    pub ty: Type,
    pub field_spec: FieldSpec,
}

enum Member<'a> {
    Field(Field),
    Code(&'a Vec<String>),
}

impl Field {
    pub fn new(modifier: ast::Modifier, name: String, ty: Type, field_spec: FieldSpec) -> Field {
        Field {
            modifier: modifier,
            name: name,
            ty: ty,
            field_spec: field_spec,
        }
    }
}

pub struct ProcessorOptions {
    parent: Options,
    pub nullable: bool,
    pub immutable: bool,
    pub build_setters: bool,
    pub build_getters: bool,
    pub build_constructor: bool,
    pub build_hash_code: bool,
    pub build_equals: bool,
    pub build_to_string: bool,
}

impl ProcessorOptions {
    pub fn new(options: Options) -> ProcessorOptions {
        ProcessorOptions {
            parent: options,
            nullable: false,
            immutable: true,
            build_setters: true,
            build_getters: true,
            build_constructor: true,
            build_hash_code: true,
            build_equals: true,
            build_to_string: true,
        }
    }
}

pub struct Processor {
    options: ProcessorOptions,
    env: Environment,
    listeners: Box<Listeners>,
    package_prefix: Option<ast::Package>,
    lower_to_upper_camel: Box<naming::Naming>,
    null_string: Variable,
    suppress_warnings: ClassType,
    string_builder: ClassType,
    override_: ClassType,
    objects: ClassType,
    object: ClassType,
    list: ClassType,
    map: ClassType,
    string: ClassType,
    optional: ClassType,
    illegal_argument: ClassType,
}

impl Processor {
    pub fn new(options: ProcessorOptions,
               env: Environment,
               package_prefix: Option<ast::Package>,
               listeners: Box<Listeners>)
               -> Processor {
        Processor {
            options: options,
            env: env,
            package_prefix: package_prefix,
            lower_to_upper_camel: naming::CamelCase::new().to_upper_camel(),
            null_string: Variable::String("null".to_owned()),
            listeners: listeners,
            override_: Type::class("java.lang", "Override"),
            objects: Type::class("java.util", "Objects"),
            suppress_warnings: Type::class("java.lang", "SuppressWarnings"),
            string_builder: Type::class("java.lang", "StringBuilder"),
            object: Type::class("java.lang", "Object"),
            list: Type::class("java.util", "List"),
            map: Type::class("java.util", "Map"),
            string: Type::class("java.lang", "String"),
            optional: Type::class("java.util", "Optional"),
            illegal_argument: Type::class("java.lang", "IllegalArgumentException"),
        }
    }

    fn field_mods(&self) -> Modifiers {
        let mut modifiers = java_mods![Modifier::Private];

        if self.options.immutable {
            modifiers.insert(Modifier::Final);
        }

        modifiers
    }

    /// Create a new FileSpec from the given package.
    fn new_file_spec(&self, package: &ast::Package) -> FileSpec {
        FileSpec::new(&self.java_package_name(package))
    }

    fn new_field_spec(&self, ty: &Type, name: &str) -> FieldSpec {
        let mods = self.field_mods();
        FieldSpec::new(mods, ty, name)
    }

    /// Build the java package of a given package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    fn java_package(&self, package: &ast::Package) -> ast::Package {
        self.package_prefix
            .clone()
            .map(|prefix| prefix.join(package))
            .unwrap_or_else(|| package.clone())
    }

    fn java_package_name(&self, package: &ast::Package) -> String {
        self.java_package(package).parts.join(".")
    }

    /// Convert the given type to a java type.
    fn convert_type(&self, package: &ast::Package, ty: &ast::Type) -> Result<Type> {
        let ty = match *ty {
            ast::Type::String => self.string.clone().into(),
            ast::Type::I32 => INTEGER.into(),
            ast::Type::U32 => INTEGER.into(),
            ast::Type::I64 => LONG.into(),
            ast::Type::U64 => LONG.into(),
            ast::Type::Float => FLOAT.into(),
            ast::Type::Double => DOUBLE.into(),
            ast::Type::Array(ref ty) => {
                let argument = self.convert_type(package, ty)?;
                self.list.with_arguments(vec![argument]).into()
            }
            ast::Type::Custom(ref string) => {
                let key = (package.clone(), string.clone());
                let _ = self.env.types.get(&key);
                let package_name = self.java_package_name(package);
                Type::class(&package_name, string).into()
            }
            ast::Type::Any => self.object.clone().into(),
            ast::Type::UsedType(ref used, ref custom) => {
                let package = self.env.lookup_used(package, used)?;
                let package_name = self.java_package_name(package);
                Type::class(&package_name, custom).into()
            }
            ast::Type::Map(ref key, ref value) => {
                let key = self.convert_type(package, key)?;
                let value = self.convert_type(package, value)?;
                self.map.with_arguments(vec![key, value]).into()
            }
            ref t => {
                return Err(format!("Unsupported type: {:?}", t).into());
            }
        };

        Ok(ty)
    }

    fn build_constructor<C>(&self, class: &C) -> ConstructorSpec
        where C: ClassLike
    {
        let mut constructor = ConstructorSpec::new(java_mods![Modifier::Public]);

        for field in class.fields() {
            let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &field.name);
            constructor.push_argument(&argument);

            if !self.options.nullable {
                if let Some(non_null) = self.require_non_null(&field, &argument) {
                    constructor.push(non_null);
                }
            }

            constructor.push(java_stmt!["this.", &field.name, " = ", argument, ";"]);
        }

        constructor
    }

    /// Build a require-non-null check.
    fn require_non_null(&self, field: &FieldSpec, argument: &ArgumentSpec) -> Option<Statement> {
        match field.ty {
            Type::Primitive(_) => None,
            _ => {
                let require_non_null = java_stmt![&self.objects, ".requireNonNull"];
                let string = Variable::String(field.name.clone());
                Some(java_stmt![require_non_null, "(", &argument, ", ", string, ");"])
            }
        }
    }

    fn build_setter(&self, field: &FieldSpec) -> Result<MethodSpec> {
        let name = format!("set{}", self.lower_to_upper_camel.convert(&field.name));
        let mut method_spec = MethodSpec::new(java_mods![Modifier::Public], &name);

        let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &field.name);

        method_spec.push_argument(&argument);
        method_spec.returns(VOID);

        let mut method_body = Elements::new();

        if !self.options.nullable {
            if let Some(non_null) = self.require_non_null(&field, &argument) {
                method_body.push(non_null);
            }
        }

        method_body.push(java_stmt!["this.", field, " = ", &argument, ";"]);

        method_spec.push(method_body);

        Ok(method_spec)
    }

    pub fn build_getter(&self, field: &FieldSpec) -> Result<MethodSpec> {
        let return_type = &field.ty;
        let name = format!("get{}", self.lower_to_upper_camel.convert(&field.name));
        let mut method_spec = MethodSpec::new(java_mods![Modifier::Public], &name);

        method_spec.returns(return_type);
        method_spec.push(java_stmt!["return this.", field, ";"]);

        Ok(method_spec)
    }

    fn build_hash_code<C>(&self, class: &C) -> MethodSpec
        where C: ClassLike
    {
        let mut hash_code = MethodSpec::new(java_mods![Modifier::Public], "hashCode");

        hash_code.push_annotation(&self.override_);
        hash_code.returns(INTEGER);

        let mut method_body = Elements::new();

        method_body.push("int result = 1;");

        for field in class.fields() {
            let field_stmt = java_stmt!["this.", field];

            let value = match field.ty {
                Type::Primitive(ref primitive) => {
                    if *primitive == INTEGER {
                        field_stmt.clone()
                    } else {
                        java_stmt![primitive.as_boxed(), ".hashCode(", &field_stmt, ")"]
                    }
                }
                _ => java_stmt![&field_stmt, ".hashCode()"],
            };

            let value = if self.options.nullable {
                match field.ty {
                    Type::Primitive(_) => value,
                    _ => java_stmt!["(", &field_stmt, " != null ? 0 : ", value, ")"],
                }
            } else {
                value
            };

            method_body.push(java_stmt!["result = result * 31 + ", value, ";"]);
        }

        method_body.push("return result;");

        hash_code.push(&method_body);

        hash_code
    }

    fn build_equals<C>(&self, class_type: &ClassType, class: &C) -> MethodSpec
        where C: ClassLike
    {
        let mut equals = MethodSpec::new(java_mods![Modifier::Public], "equals");

        equals.push_annotation(&self.override_);
        equals.returns(BOOLEAN);

        let argument = ArgumentSpec::new(java_mods![Modifier::Final], &self.object, "other");

        equals.push_argument(&argument);

        // check if argument is null.
        {
            let mut null_check = Elements::new();

            null_check.push(java_stmt!["if (", &argument, " == null) {"]);
            null_check.push_nested("return false;");
            null_check.push("}");

            equals.push(null_check);
        }

        // check that argument is expected type.
        {
            let mut instanceof_check = Elements::new();

            instanceof_check.push(java_stmt!["if (!(", &argument, " instanceof ", class_type, ")) {"]);
            instanceof_check.push_nested("return false;");
            instanceof_check.push("}");

            equals.push(instanceof_check);
        }

        // cast argument.
        let o = java_stmt!["o"];

        let mut cast = Elements::new();

        let mut suppress_warnings = AnnotationSpec::new(&self.suppress_warnings);
        suppress_warnings.push_argument(Variable::String("unchecked".to_owned()));

        cast.push(suppress_warnings);
        cast.push(java_stmt!["final ", class_type, " ", &o, " = (", class_type, ") ", argument,
                             ";"]);

        equals.push(cast);

        for field in class.fields() {
            let field_stmt = java_stmt!["this.", field];
            let o = java_stmt![&o, ".", &field.name];

            let equals_condition = match field.ty {
                Type::Primitive(_) => java_stmt![&field_stmt, " != ", &o],
                _ => java_stmt!["!", &field_stmt, ".equals(", &o, ")"],
            };

            let mut equals_check = Elements::new();

            equals_check.push(java_stmt!["if (", equals_condition, ") {"]);
            equals_check.push_nested("return false;");
            equals_check.push("}");

            if self.options.nullable {
                let mut null_check = Elements::new();

                null_check.push(java_stmt!["if (", &o, " != null) {"]);
                null_check.push_nested("return false;");
                null_check.push("}");

                let mut field_check = Elements::new();

                field_check.push(java_stmt!["if (", &field_stmt, " == null) {"]);
                field_check.push_nested(null_check);
                field_check.push("} else {");
                field_check.push_nested(equals_check);
                field_check.push("}");

                equals.push(field_check);
            } else {
                equals.push(equals_check);
            }
        }

        equals.push("return true;");

        equals
    }

    fn build_to_string<C>(&self, class_type: &ClassType, class: &C) -> MethodSpec
        where C: ClassLike
    {
        let mut to_string = MethodSpec::new(java_mods![Modifier::Public], "toString");

        to_string.push_annotation(&self.override_);
        to_string.returns(&self.string);

        let b = java_stmt!["b"];

        let new_string_builder = java_stmt!["new ", &self.string_builder, "();"];

        to_string.push(java_stmt!["final ",
                                  &self.string_builder,
                                  " ",
                                  &b,
                                  " = ", &new_string_builder]);

        let mut fields = Elements::new();

        for field in class.fields() {
            let mut field_append = Elements::new();

            let field_stmt = java_stmt!["this.", field];

            let format = match field.ty {
                Type::Primitive(ref primitive) => {
                    java_stmt![primitive.as_boxed(), ".toString(", &field_stmt, ")"]
                }
                _ => {
                    let format = java_stmt![&field_stmt, ".toString()"];

                    if self.options.nullable {
                        java_stmt![&field_stmt, " == null ? ", &self.null_string, " : ", format]
                    } else {
                        format
                    }
                }
            };

            let field_key = Variable::String(format!("{} = ", &field.name));

            field_append.push(java_stmt![&b, ".append(", field_key, ");"]);
            field_append.push(java_stmt![&b, ".append(", format, ");"]);

            fields.push(field_append);
        }

        /// join each field with ", "
        let field_joiner = java_stmt![&b, ".append(", Variable::String(", ".to_owned()), ");"];

        let mut class_appends = Elements::new();

        class_appends.push(java_stmt![&b, ".append(", Variable::String(class_type.name.clone()), ");"]);
        class_appends.push(java_stmt![&b, ".append(", Variable::String("(".to_owned()), ");"]);
        class_appends.push(fields.join(field_joiner));
        class_appends.push(java_stmt![&b, ".append(", Variable::String(")".to_owned()), ");"]);

        to_string.push(class_appends);
        to_string.push(java_stmt!["return ", &b, ".toString();"]);

        to_string
    }

    fn add_class_like<C>(&self, class_type: &ClassType, class: &mut C) -> Result<()>
        where C: ClassLike + ContainerSpec
    {
        for field in class.fields().clone() {
            if self.options.build_getters {
                let getter = self.build_getter(&field)?;
                class.push(getter);
            }

            if self.options.build_setters {
                if !field.modifiers.contains(&Modifier::Final) {
                    let setter = self.build_setter(&field)?;
                    class.push(setter);
                }
            }
        }

        if self.options.build_hash_code {
            let hash_code = self.build_hash_code(class);
            class.push(hash_code);
        }

        if self.options.build_equals {
            let equals = self.build_equals(class_type, class);
            class.push(equals);
        }

        if self.options.build_to_string {
            let to_string = self.build_to_string(class_type, class);
            class.push(to_string);
        }

        Ok(())
    }

    fn add_class<C>(&self, class_type: &ClassType, class: &mut C) -> Result<()>
        where C: ClassLike + ContainerSpec
    {
        if self.options.build_constructor {
            let constructor = self.build_constructor(class);
            class.push_constructor(constructor);
        }

        self.add_class_like(class_type, class)
    }

    fn build_enum_constructor(&self, en: &EnumSpec) -> ConstructorSpec {
        let mut constructor = ConstructorSpec::new(java_mods![Modifier::Private]);

        for field in &en.fields {
            let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &field.name);
            constructor.push_argument(&argument);

            if !self.options.nullable {
                if let Some(non_null) = self.require_non_null(&field, &argument) {
                    constructor.push(non_null);
                }
            }

            constructor.push(java_stmt!["this.", &field.name, " = ", argument, ";"]);
        }

        constructor
    }

    fn to_integer_literal(&self, number: &i64, ty: &PrimitiveType) -> Result<String> {
        match *ty {
            INTEGER => Ok(number.to_string()),
            LONG => Ok(format!("{}L", number.to_string())),
            _ => Err(format!("Cannot convert integer ({}) to {}", number, ty.primitive).into()),
        }
    }

    fn to_float_literal(&self, number: &f64, ty: &PrimitiveType) -> Result<String> {
        match *ty {
            DOUBLE => Ok(format!("{}D", number.to_string())),
            FLOAT => Ok(format!("{}F", number.to_string())),
            _ => Err(format!("Cannot convert float ({}) to {}", number, ty.primitive).into()),
        }
    }

    fn literal_value(&self, value: &ast::Value, ty: &Type) -> Result<Variable> {

        if let Type::Primitive(ref primitive) = *ty {
            if let ast::Value::Integer(ref integer) = *value {
                let lit = self.to_integer_literal(integer, primitive)?;
                return Ok(lit.into());
            }

            if let ast::Value::Float(ref float) = *value {
                let lit = self.to_float_literal(float, primitive)?;
                return Ok(lit.into());
            }
        }

        if let Type::Class(ref class) = *ty {
            if *class == self.string {
                if let ast::Value::String(ref value) = *value {
                    return Ok(Variable::String(value.to_owned()));
                }
            }
        }

        Err(format!("{} cannot be applied to type {}", value, display_type(ty)).into())
    }

    fn find_field(&self, fields: &Vec<Field>, name: &str) -> Result<Field> {
        for field in fields {
            if field.name == name {
                return Ok(field.clone());
            }
        }

        Err(format!("no field named: {}", name).into())
    }

    fn enum_from_value_method(&self, field: &Field, class_type: &ClassType) -> Result<MethodSpec> {
        let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &field.name);

        let value = java_stmt!["value"];

        let cond = match field.ty {
            Type::Primitive(_) => java_stmt![&value, ".", &field.name, " == ", &argument],
            _ => java_stmt![&value, ".", &field.name, ".equals(", &argument, ")"],
        };

        let mut return_matched = Elements::new();

        return_matched.push(java_stmt!["if (", &cond, ") {"]);
        return_matched.push_nested(java_stmt!["return ", &value, ";"]);
        return_matched.push("}");

        let mut value_loop = Elements::new();

        value_loop.push(java_stmt!["for (final ", class_type, " ", &value, " : ", "values()) {"]);

        value_loop.push_nested(return_matched);
        value_loop.push("}");

        let mut from_value = MethodSpec::new(java_mods![Modifier::Public, Modifier::Static],
                                             "fromValue");

        let argument_name = Variable::String(argument.name.clone());
        let throw = java_stmt!["throw new ", &self.illegal_argument, "(", argument_name, ");"];

        from_value.returns(class_type);
        from_value.push_argument(argument);
        from_value.push(value_loop);
        from_value.push(throw);

        Ok(from_value)
    }

    fn enum_to_value_method(&self, field: &Field) -> Result<MethodSpec> {
        let mut to_value = MethodSpec::new(java_mods![Modifier::Public], "toValue");

        to_value.returns(&field.ty);
        to_value.push(java_stmt!["return this.", &field.name, ";"]);

        Ok(to_value)
    }

    fn process_enum(&self, package: &ast::Package, ty: &ast::EnumBody) -> Result<FileSpec> {
        let class_type = Type::class(&self.java_package_name(package), &ty.name);

        let mut en = EnumSpec::new(java_mods![Modifier::Public], &ty.name);
        let mut fields = Vec::new();

        self.process_members(package, &ty.members, |m| {
                match m {
                    Member::Field(field) => {
                        en.push_field(&field.field_spec);
                        fields.push(field);
                    }
                    Member::Code(code) => {
                        en.push(code);
                    }
                }
            })?;

        for enum_literal in &ty.values {
            let mut enum_value = Elements::new();
            let mut enum_stmt = java_stmt![&enum_literal.name];

            if !enum_literal.values.is_empty() {
                let mut value_arguments = Statement::new();

                for (value, field) in enum_literal.values.iter().zip(fields.iter()) {
                    value_arguments.push(self.literal_value(value, &field.ty)?);
                }

                enum_stmt.push(java_stmt!["(", value_arguments.join(", "), ")"]);
            }

            enum_value.push(enum_stmt);
            en.push_value(enum_value);
        }

        let constructor = self.build_enum_constructor(&en);
        en.push_constructor(constructor);

        for field in en.fields.clone() {
            if self.options.build_getters {
                let getter = self.build_getter(&field)?;
                en.push(getter);
            }
        }

        let mut from_value: Option<MethodSpec> = None;
        let mut to_value: Option<MethodSpec> = None;

        if let Some(serialize_as) = ty.options.lookup_identifier_nth("serialize_as", 0) {
            let field = self.find_field(&fields, serialize_as)?;

            from_value = Some(self.enum_from_value_method(&field, &class_type)?);
            to_value = Some(self.enum_to_value_method(&field)?);
        }

        self.listeners
            .enum_added(ty,
                        &fields,
                        &class_type,
                        &mut from_value,
                        &mut to_value,
                        &mut en)?;

        if let Some(from_value) = from_value {
            en.push(from_value);
        }

        if let Some(to_value) = to_value {
            en.push(to_value);
        }

        let mut file_spec = self.new_file_spec(package);
        file_spec.push(&en);

        Ok(file_spec)
    }

    fn process_tuple(&self, package: &ast::Package, ty: &ast::TupleBody) -> Result<FileSpec> {
        let class_type = Type::class(&self.java_package_name(package), &ty.name);

        let mut class = ClassSpec::new(java_mods![Modifier::Public], &ty.name);
        let mut fields = Vec::new();

        self.process_members(package, &ty.members, |m| {
                match m {
                    Member::Field(field) => {
                        class.push_field(&field.field_spec);
                        fields.push(field);
                    }
                    Member::Code(code) => {
                        class.push(code);
                    }
                }
            })?;

        self.add_class(&class_type, &mut class)?;
        self.listeners.class_added(&fields, &class_type, &mut class)?;

        let mut file_spec = self.new_file_spec(package);
        file_spec.push(&class);

        Ok(file_spec)
    }

    fn process_type(&self, package: &ast::Package, message: &ast::TypeBody) -> Result<FileSpec> {
        let class_type = Type::class(&self.java_package_name(package), &message.name);

        let mut class = ClassSpec::new(java_mods![Modifier::Public], &message.name);
        let mut fields = Vec::new();

        self.process_members(package, &message.members, |m| {
                match m {
                    Member::Field(field) => {
                        class.push_field(&field.field_spec);
                        fields.push(field);
                    }
                    Member::Code(code) => {
                        class.push(code);
                    }
                }
            })?;


        self.add_class(&class_type, &mut class)?;
        self.listeners.class_added(&fields, &class_type, &mut class)?;

        let mut file_spec = self.new_file_spec(package);
        file_spec.push(&class);

        Ok(file_spec)
    }

    fn process_interface(&self,
                         package: &ast::Package,
                         interface: &ast::InterfaceBody)
                         -> Result<FileSpec> {
        let parent_type = Type::class(&self.java_package_name(package), &interface.name);

        let mut interface_spec = InterfaceSpec::new(java_mods![Modifier::Public], &interface.name);
        let mut interface_fields: Vec<Field> = Vec::new();

        self.process_members(package, &interface.members, |m| {
                match m {
                    Member::Field(field) => {
                        interface_fields.push(field);
                    }
                    Member::Code(code) => {
                        interface_spec.push(code);
                    }
                }
            })?;

        for (_, ref sub_type) in &interface.sub_types {
            let class_type = parent_type.extend(&sub_type.name);

            let mods = java_mods![Modifier::Public, Modifier::Static];
            let mut class = ClassSpec::new(mods, &sub_type.name);
            let mut fields = interface_fields.clone();

            class.implements(&parent_type);

            for interface_field in &interface_fields {
                class.push_field(&interface_field.field_spec);
            }

            self.process_members(package, &sub_type.members, |m| {
                    match m {
                        Member::Field(field) => {
                            class.push_field(&field.field_spec);
                            fields.push(field);
                        }
                        Member::Code(code) => {
                            class.push(code);
                        }
                    }
                })?;

            self.add_class(&class_type, &mut class)?;
            self.listeners.class_added(&fields, &class_type, &mut class)?;

            self.listeners.sub_type_added(&fields, interface, sub_type, &mut class)?;
            interface_spec.push(&class);
        }

        let mut file_spec = self.new_file_spec(package);

        self.listeners.interface_added(interface, &mut interface_spec)?;

        file_spec.push(&interface_spec);
        Ok(file_spec)
    }

    fn process_members<C>(&self,
                          package: &ast::Package,
                          members: &Vec<ast::Member>,
                          mut consumer: C)
                          -> Result<()>
        where C: FnMut(Member) -> ()
    {
        for member in members {
            if let ast::Member::Field(ref field, _) = *member {
                let field_type = self.convert_type(package, &field.ty)?;
                let field_spec = self.push_field(&field_type, field)?;

                let field = Field::new(field.modifier.clone(),
                                       field.name.clone(),
                                       field_type,
                                       field_spec);

                consumer(Member::Field(field));
                continue;
            }

            if let ast::Member::Code(ref context, ref content, _) = *member {
                if context == JAVA_CONTEXT {
                    consumer(Member::Code(content));
                }

                continue;
            }
        }

        Ok(())
    }

    fn push_field(&self, field_type: &Type, field: &ast::Field) -> Result<FieldSpec> {
        let field_type = if field.is_optional() {
            self.optional.with_arguments(vec![field_type]).into()
        } else {
            field_type.clone()
        };

        let name = if let Some(ref id_converter) = self.options.parent.id_converter {
            id_converter.convert(&field.name)
        } else {
            field.name.clone()
        };

        Ok(self.new_field_spec(&field_type, &name))
    }
}

impl Backend for Processor {
    fn process(&self) -> Result<()> {
        let root_dir = &self.options.parent.out_path;

        // Process all types discovered so far.
        for (&(ref package, _), decl) in &self.env.types {
            let out_dir = self.java_package(package)
                .parts
                .iter()
                .fold(root_dir.clone(), |current, next| current.join(next));

            if !out_dir.is_dir() {
                debug!("+dir: {}", out_dir.display());
                fs::create_dir_all(&out_dir)?;
            }

            let full_path = out_dir.join(format!("{}.java", decl.name()));

            let file_spec = match *decl {
                ast::Decl::Interface(ref interface, _) => {
                    self.process_interface(package, interface)
                }
                ast::Decl::Type(ref ty, _) => self.process_type(package, ty),
                ast::Decl::Tuple(ref ty, _) => self.process_tuple(package, ty),
                ast::Decl::Enum(ref ty, _) => self.process_enum(package, ty),
            }?;

            debug!("+class: {}", full_path.display());

            let out = file_spec.format();
            let mut f = File::create(full_path)?;
            let bytes = out.into_bytes();

            f.write_all(&bytes)?;
            f.flush()?;
        }

        Ok(())
    }
}

impl ::std::fmt::Display for ast::Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let out = match *self {
            ast::Value::String(_) => "<string>",
            ast::Value::Integer(_) => "<int>",
            ast::Value::Float(_) => "<float>",
            ast::Value::Identifier(_) => "<identifier>",
            ast::Value::Type(_) => "<type>",
        };

        write!(f, "{}", out)
    }
}

fn display_type(ty: &Type) -> String {
    match *ty {
        Type::Primitive(ref primitive) => primitive.primitive.to_owned(),
        Type::Class(ref class) => format!("class {}.{}", class.package, class.name),
        _ => "<unknown>".to_owned(),
    }
}
