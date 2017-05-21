use backend::Backend;
use environment::Environment;
use naming::{self, FromNaming};
use options::Options;
use parser::ast;
use std::fs::File;
use std::fs;
use std::io::Write;

#[macro_use]
use codegen::java::*;

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

    fn interface_added(&self,
                       _interface: &ast::InterfaceDecl,
                       _interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        Ok(())
    }

    fn sub_type_added(&self,
                      _fields: &Vec<Field>,
                      _interface: &ast::InterfaceDecl,
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

    fn interface_added(&self,
                       interface: &ast::InterfaceDecl,
                       interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        for listeners in self {
            listeners.interface_added(interface, interface_spec)?;
        }

        Ok(())
    }

    fn sub_type_added(&self,
                      fields: &Vec<Field>,
                      interface: &ast::InterfaceDecl,
                      sub_type: &ast::SubType,
                      class: &mut ClassSpec)
                      -> Result<()> {
        for listeners in self {
            listeners.sub_type_added(fields, interface, sub_type, class)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Field {
    pub modifier: ast::Modifier,
    pub name: String,
    pub ty: Type,
    pub field_spec: FieldSpec,
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
    suppress_warnings: ClassType,
    string_builder: ClassType,
    override_: ClassType,
    object: ClassType,
    list: ClassType,
    map: ClassType,
    string: ClassType,
    optional: ClassType,
    integer: PrimitiveType,
    boolean: PrimitiveType,
    long: PrimitiveType,
    float: PrimitiveType,
    double: PrimitiveType,
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
            listeners: listeners,
            override_: Type::class("java.lang", "Override"),
            suppress_warnings: Type::class("java.lang", "SuppressWarnings"),
            string_builder: Type::class("java.lang", "StringBuilder"),
            object: Type::class("java.lang", "Object"),
            list: Type::class("java.util", "List"),
            map: Type::class("java.util", "Map"),
            string: Type::class("java.lang", "String"),
            optional: Type::class("java.util", "Optional"),
            integer: Type::primitive("int", "Integer"),
            boolean: Type::primitive("boolean", "Boolean"),
            long: Type::primitive("long", "Long"),
            float: Type::primitive("float", "Float"),
            double: Type::primitive("double", "Double"),
        }
    }

    /// Create a new FileSpec from the given package.
    fn new_file_spec(&self, package: &ast::Package) -> FileSpec {
        FileSpec::new(&self.java_package_name(package))
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
            ast::Type::String => self.string.clone().as_type(),
            ast::Type::I32 => self.integer.clone().as_type(),
            ast::Type::U32 => self.integer.clone().as_type(),
            ast::Type::I64 => self.long.clone().as_type(),
            ast::Type::U64 => self.long.clone().as_type(),
            ast::Type::Float => self.float.clone().as_type(),
            ast::Type::Double => self.double.clone().as_type(),
            ast::Type::Array(ref ty) => {
                let argument = self.convert_type(package, ty)?;
                self.list.with_arguments(vec![argument]).as_type()
            }
            ast::Type::Custom(ref string) => {
                let key = (package.clone(), string.clone());
                let _ = self.env.types.get(&key);
                let package_name = self.java_package_name(package);
                Type::class(&package_name, string).as_type()
            }
            ast::Type::Any => self.object.clone().as_type(),
            ast::Type::UsedType(ref used, ref custom) => {
                let package = self.env.lookup_used(package, used)?;
                let package_name = self.java_package_name(package);
                Type::class(&package_name, custom).as_type()
            }
            ast::Type::Map(ref key, ref value) => {
                let key = self.convert_type(package, key)?;
                let value = self.convert_type(package, value)?;
                self.map.with_arguments(vec![key, value]).as_type()
            }
            ref t => {
                return Err(format!("Unsupported type: {:?}", t).into());
            }
        };

        Ok(ty)
    }

    fn build_constructor(&self, class: &ClassSpec) -> ConstructorSpec {
        let mut constructor = ConstructorSpec::new(java_mods![Modifier::Public]);

        for field in &class.fields {
            let argument = ArgumentSpec::new(java_mods![Modifier::Final], &field.ty, &field.name);
            constructor.push_argument(&argument);
            constructor.push(java_stmt!["this.", &field.name, " = ", argument, ";"]);
        }

        constructor
    }

    fn build_getters(&self, class: &ClassSpec) -> Result<Vec<MethodSpec>> {
        let mut result = Vec::new();

        for field in &class.fields {
            let return_type = &field.ty;
            let name = format!("get{}", self.lower_to_upper_camel.convert(&field.name));
            let mut method_spec = MethodSpec::new(java_mods![Modifier::Public], &name);
            method_spec.returns(return_type);
            method_spec.push(java_stmt!["return this.", &field, ";"]);
            result.push(method_spec);
        }

        Ok(result)
    }

    fn build_hash_code(&self, class: &ClassSpec) -> MethodSpec {
        let mut hash_code = MethodSpec::new(java_mods![Modifier::Public], "hashCode");

        hash_code.push_annotation(&self.override_);
        hash_code.returns(&self.integer);
        hash_code.push("int result = 17;");

        let mut mutations = Elements::new();

        for field in &class.fields {
            let field_stmt = java_stmt!["this.", field];

            let value = match field.ty {
                Type::Primitive(ref primitive) => {
                    if *primitive == self.integer {
                        field_stmt
                    } else {
                        java_stmt![primitive.as_boxed(), ".hashCode(", field_stmt, ")"]
                    }
                }
                _ => java_stmt![field_stmt, ".hashCode()"],
            };

            mutations.push(java_stmt!["result = 31 * ", value, ";"]);
        }

        hash_code.push(&mutations);
        hash_code.push("return result;");

        hash_code
    }

    fn build_equals(&self, class_type: &ClassType, class: &ClassSpec) -> MethodSpec {
        let mut equals = MethodSpec::new(java_mods![Modifier::Public], "equals");

        equals.push_annotation(&self.override_);
        equals.returns(&self.boolean);

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

        for field in &class.fields {
            let mut field_check = Elements::new();

            let field_stmt = java_stmt!["this.", field];
            let other_arg = java_stmt![&o, ".", &field.name];

            let condition = match field.ty {
                Type::Primitive(_) => java_stmt![field_stmt, " != ", other_arg],
                _ => java_stmt!["!", field_stmt, ".equals(", other_arg, ")"],
            };

            field_check.push(java_stmt!["if (", condition, ") {"]);
            field_check.push_nested("return false;");
            field_check.push("}");

            equals.push(field_check);
        }

        equals.push("return true;");

        equals
    }

    fn build_to_string(&self, class_type: &ClassType, class: &ClassSpec) -> MethodSpec {
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

        for field in &class.fields {
            let mut field_append = Elements::new();

            let field_stmt = java_stmt!["this.", field];

            let format = match field.ty {
                Type::Primitive(ref primitive) => {
                    java_stmt![primitive.as_boxed(), ".toString(", field_stmt, ")"]
                }
                _ => java_stmt![field_stmt, ".toString()"],
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

    fn add_class(&self,
                 fields: &Vec<Field>,
                 class_type: &ClassType,
                 class: &mut ClassSpec)
                 -> Result<()> {
        if self.options.build_constructor {
            let constructor = self.build_constructor(class);
            class.push_constructor(constructor);
        }

        if self.options.build_getters {
            for getter in self.build_getters(&class)? {
                class.push(getter);
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

        self.listeners.class_added(&fields, &class_type, class)?;

        Ok(())
    }

    fn process_type(&self, package: &ast::Package, ty: &ast::TypeDecl) -> Result<FileSpec> {
        let class_type = Type::class(&self.java_package_name(package), &ty.name);

        let mut class = ClassSpec::new(java_mods![Modifier::Public], &ty.name);
        let mut fields = Vec::new();

        match ty.value {
            ast::Type::Tuple(ref elements) => {
                for (index, element) in elements.iter().enumerate() {
                    let field_type = self.convert_type(package, &element.ty)?;
                    let mods = java_mods![Modifier::Private, Modifier::Final];

                    let index_name = match index {
                        0 => "first".to_owned(),
                        1 => "second".to_owned(),
                        2 => "third".to_owned(),
                        n => format!("field{}", n),
                    };

                    let name = element.name.clone().unwrap_or(index_name);
                    let field_spec = FieldSpec::new(mods, &field_type, &name);
                    class.push_field(&field_spec);
                    fields.push(Field::new(ast::Modifier::Required, name, field_type, field_spec));
                }
            }
            _ => {}
        }

        self.add_class(&fields, &class_type, &mut class)?;

        let mut file_spec = self.new_file_spec(package);
        file_spec.push(&class);

        Ok(file_spec)
    }

    fn process_message(&self,
                       package: &ast::Package,
                       message: &ast::MessageDecl)
                       -> Result<FileSpec> {
        let class_type = Type::class(&self.java_package_name(package), &message.name);

        let mut class = ClassSpec::new(java_mods![Modifier::Public], &message.name);
        let mut fields = Vec::new();

        for member in &message.members {
            if let ast::MessageMember::Field(ref field, _) = *member {
                let field_type = self.convert_type(package, &field.ty)?;
                let field_spec = self.push_field(&field_type, field)?;

                class.push_field(&field_spec);

                fields.push(Field::new(field.modifier.clone(),
                                       field.name.clone(),
                                       field_type,
                                       field_spec));

                continue;
            }

            if let ast::MessageMember::Code(ref context, ref content, _) = *member {
                if context == JAVA_CONTEXT {
                    class.push(content);
                }

                continue;
            }
        }

        self.add_class(&fields, &class_type, &mut class)?;

        let mut file_spec = self.new_file_spec(package);
        file_spec.push(&class);

        Ok(file_spec)
    }

    fn process_interface(&self,
                         package: &ast::Package,
                         interface: &ast::InterfaceDecl)
                         -> Result<FileSpec> {
        let parent_type = Type::class(&self.java_package_name(package), &interface.name);

        let mut interface_spec = InterfaceSpec::new(java_mods![Modifier::Public], &interface.name);
        let mut interface_fields = Vec::new();

        for member in &interface.members {
            if let ast::InterfaceMember::Field(ref field, _) = *member {
                let field_type = self.convert_type(package, &field.ty)?;
                let field_spec = self.push_field(&field_type, field)?;

                interface_fields.push(Field::new(field.modifier.clone(),
                                                 field.name.clone(),
                                                 field_type,
                                                 field_spec));
                continue;
            }

            if let ast::InterfaceMember::Code(ref context, ref content, _) = *member {
                if context == JAVA_CONTEXT {
                    interface_spec.push(content);
                }

                continue;
            }
        }

        for (_, ref sub_type) in &interface.sub_types {
            let class_type = parent_type.extend(&sub_type.name);

            let mods = java_mods![Modifier::Public, Modifier::Static];
            let mut class = ClassSpec::new(mods, &sub_type.name);
            let mut fields = interface_fields.clone();

            class.implements(&parent_type);

            for interface_field in &interface_fields {
                class.push_field(&interface_field.field_spec);
            }

            for member in &sub_type.members {
                if let ast::SubTypeMember::Field(ref field) = *member {
                    let field_type = self.convert_type(package, &field.ty)?;
                    let field_spec = self.push_field(&field_type, field)?;

                    class.push_field(&field_spec);
                    fields.push(Field::new(field.modifier.clone(),
                                           field.name.clone(),
                                           field_type,
                                           field_spec));
                    continue;
                }

                if let ast::SubTypeMember::Code(ref context, ref content, _) = *member {
                    if context == JAVA_CONTEXT {
                        class.push(content);
                    }

                    continue;
                }
            }

            self.add_class(&fields, &class_type, &mut class)?;

            self.listeners.sub_type_added(&fields, interface, sub_type, &mut class)?;
            interface_spec.push(&class);
        }

        let mut file_spec = self.new_file_spec(package);

        self.listeners.interface_added(interface, &mut interface_spec)?;

        file_spec.push(&interface_spec);
        Ok(file_spec)
    }

    fn push_field(&self, field_type: &Type, field: &ast::Field) -> Result<FieldSpec> {
        let field_type = if field.is_optional() {
            self.optional.with_arguments(vec![field_type]).as_type()
        } else {
            field_type.clone()
        };

        let mods = java_mods![Modifier::Private, Modifier::Final];

        let name = if let Some(ref id_converter) = self.options.parent.id_converter {
            id_converter.convert(&field.name)
        } else {
            field.name.clone()
        };

        let field = FieldSpec::new(mods, &field_type, &name);

        Ok(field)
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
                ast::Decl::Interface(ref interface) => self.process_interface(package, interface),
                ast::Decl::Message(ref message) => self.process_message(package, message),
                ast::Decl::Type(ref ty) => self.process_type(package, ty),
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
