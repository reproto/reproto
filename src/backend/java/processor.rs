use environment::Environment;
use options::Options;
use parser::ast;
use std::fs::File;
use std::fs;
use std::io::Write;

#[macro_use]
use codegen::java::*;

use errors::*;

pub trait Listeners {
    fn class_added(&self, _class: &mut ClassSpec) -> Result<()> {
        Ok(())
    }

    fn interface_added(&self,
                       _interface: &ast::InterfaceDecl,
                       _interface_spec: &mut InterfaceSpec)
                       -> Result<()> {
        Ok(())
    }

    fn sub_type_added(&self,
                      _interface: &ast::InterfaceDecl,
                      _sub_type: &ast::SubType,
                      _class: &mut ClassSpec)
                      -> Result<()> {
        Ok(())
    }
}

pub struct Processor<'a> {
    options: &'a Options,
    env: &'a Environment,
    package_prefix: Option<ast::Package>,
    object: Type,
    list: Type,
    string: Type,
    integer: Type,
    long: Type,
    float: Type,
    double: Type,
}

impl<'a> Processor<'a> {
    pub fn new(options: &'a Options,
               env: &'a Environment,
               package_prefix: Option<ast::Package>)
               -> Processor<'a> {
        Processor {
            options: options,
            env: env,
            package_prefix: package_prefix,
            object: Type::new("java.lang", "Object"),
            list: Type::new("java.util", "List"),
            string: Type::new("java.lang", "String"),
            integer: Type::new("java.lang", "Integer"),
            long: Type::new("java.lang", "Long"),
            float: Type::new("java.lang", "Float"),
            double: Type::new("java.lang", "Double"),
        }
    }

    /// Create a new FileSpec from the given package.
    fn new_file_spec(&self, package: &ast::Package) -> FileSpec {
        let package_name = self.java_package(package).parts.join(".");
        FileSpec::new(&package_name)
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

    /// Convert the given type to a java type.
    fn convert_type(&self, package: &ast::Package, type_: &ast::Type) -> Result<TypeSpec> {
        let type_ = match *type_ {
            ast::Type::String => self.string.as_type_spec(),
            ast::Type::I32 => self.integer.as_type_spec(),
            ast::Type::U32 => self.integer.as_type_spec(),
            ast::Type::I64 => self.long.as_type_spec(),
            ast::Type::U64 => self.long.as_type_spec(),
            ast::Type::Array(ref type_) => {
                let argument = self.convert_type(package, type_)?;
                self.list.with_arguments(vec![argument])
            }
            ast::Type::Custom(ref string) => {
                let key = (package.clone(), string.clone());
                let _ = self.env.types.get(&key);
                let package_name = self.java_package(package).parts.join(".");
                Type::new(&package_name, string).as_type_spec()
            }
            ast::Type::Any => self.object.as_type_spec(),
            ast::Type::Float => self.float.as_type_spec(),
            ast::Type::Double => self.double.as_type_spec(),
            ast::Type::UsedType(ref used, ref custom) => {
                let package = self.env.lookup_used(package, used)?;
                let package_name = self.java_package(package).parts.join(".");
                Type::new(&package_name, custom).as_type_spec()
            }
            ref t => {
                return Err(format!("Unsupported type: {:?}", t).into());
            }
        };

        Ok(type_)
    }

    fn build_constructor(&self, class: &ClassSpec) -> ConstructorSpec {
        let mut constructor = ConstructorSpec::new(mods![Modifier::Public]);

        for field in &class.fields {
            let argument = ArgumentSpec::new(mods![Modifier::Final], &field.type_, &field.name);
            constructor.push_argument(&argument);
            constructor.push_statement(&stmt!["this.$L = $N", literal field.name.clone(), name argument]);
        }

        constructor
    }

    fn process_type<L>(&self,
                       package: &ast::Package,
                       type_: &ast::TypeDecl,
                       listeners: &L)
                       -> Result<FileSpec>
        where L: Listeners
    {
        let mut class = ClassSpec::new(mods![Modifier::Public], &type_.name);

        match type_.value {
            ast::Type::Tuple(ref arguments) => {
                let mut index = 0;

                for argument in arguments {
                    let field_type = self.convert_type(package, argument)?;
                    let mods = mods![Modifier::Private, Modifier::Final];

                    let name = match index {
                        0 => "first".to_owned(),
                        1 => "second".to_owned(),
                        2 => "third".to_owned(),
                        n => format!("field{}", n),
                    };

                    index += 1;

                    let field = FieldSpec::new(mods, &field_type, &name);
                    class.push_field(&field);
                }
            }
            _ => {}
        }

        let constructor = self.build_constructor(&class);
        class.push_constructor(&constructor);

        listeners.class_added(&mut class)?;

        let mut file_spec = self.new_file_spec(package);
        file_spec.push_class(&class);

        Ok(file_spec)
    }

    fn process_message<L>(&self,
                          package: &ast::Package,
                          message: &ast::MessageDecl,
                          listeners: &L)
                          -> Result<FileSpec>
        where L: Listeners
    {
        let mut class = ClassSpec::new(mods![Modifier::Public], &message.name);

        for member in &message.members {
            if let ast::MessageMember::Field(ref field, _) = *member {
                class.push_field(&self.push_field(&package, field)?);
            }
        }

        let constructor = &self.build_constructor(&class);
        class.push_constructor(constructor);

        listeners.class_added(&mut class)?;

        let mut file_spec = self.new_file_spec(package);
        file_spec.push_class(&class);

        Ok(file_spec)
    }

    fn process_interface<L>(&self,
                            package: &ast::Package,
                            interface: &ast::InterfaceDecl,
                            listeners: &L)
                            -> Result<FileSpec>
        where L: Listeners
    {
        let mut interface_spec = InterfaceSpec::new(mods![Modifier::Public], &interface.name);

        let mut interface_fields: Vec<FieldSpec> = Vec::new();

        for member in &interface.members {
            if let ast::InterfaceMember::Field(ref field, _) = *member {
                let field = self.push_field(&package, field)?;
                interface_fields.push(field);
            }
        }

        for (_, ref sub_type) in &interface.sub_types {
            let mods = mods![Modifier::Public, Modifier::Static];
            let mut class = ClassSpec::new(mods, &sub_type.name);

            for interface_field in &interface_fields {
                class.push_field(&interface_field);
            }

            for member in &sub_type.members {
                if let ast::SubTypeMember::Field(ref field) = *member {
                    let field = self.push_field(&package, field)?;
                    class.push_field(&field);
                }
            }

            let constructor = self.build_constructor(&class);
            class.push_constructor(&constructor);

            listeners.class_added(&mut class)?;
            listeners.sub_type_added(interface, sub_type, &mut class)?;

            interface_spec.push_class(&class);
        }

        let mut file_spec = self.new_file_spec(package);

        listeners.interface_added(interface, &mut interface_spec)?;

        file_spec.push_interface(&interface_spec);
        Ok(file_spec)
    }

    fn push_field(&self, package: &ast::Package, field: &ast::Field) -> Result<FieldSpec> {
        let field_type = self.convert_type(package, &field.type_)?;
        let mods = mods![Modifier::Private, Modifier::Final];
        let field = FieldSpec::new(mods, &field_type, &field.name);

        Ok(field)
    }

    pub fn process<L>(&self, listeners: &L) -> Result<()>
        where L: Listeners
    {
        let root_dir = &self.options.out_path;

        // Create target directory.
        if !root_dir.is_dir() {
            info!("Creating: {}", root_dir.display());
            fs::create_dir_all(root_dir)?;
        }

        // Process all types discovered so far.
        for (&(ref package, _), decl) in &self.env.types {
            let out_dir = self.java_package(package)
                .parts
                .iter()
                .fold(root_dir.clone(), |current, next| current.join(next));

            fs::create_dir_all(&out_dir)?;

            let full_path = out_dir.join(format!("{}.java", decl.name()));

            let file_spec = match *decl {
                ast::Decl::Interface(ref interface) => {
                    self.process_interface(package, interface, listeners)
                }
                ast::Decl::Message(ref message) => {
                    self.process_message(package, message, listeners)
                }
                ast::Decl::Type(ref type_) => self.process_type(package, type_, listeners),
            }?;

            debug!("Writing: {}", full_path.display());

            let out = file_spec.format()?;
            let mut f = File::create(full_path)?;
            let bytes = out.into_bytes();

            f.write_all(&bytes)?;
            f.flush()?;
        }

        Ok(())
    }
}
