use environment::Environment;
use options::Options;
use parser::ast;
use std::fs::File;
use std::fs;
use std::io::Write;
use naming::{self, FromNaming};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[macro_use]
use codegen::python::*;

use errors::*;

pub trait Listeners {}

pub struct Processor<'a> {
    options: &'a Options,
    env: &'a Environment,
    package_prefix: Option<ast::Package>,
    to_lower_snake: Box<naming::Naming>,
}

const PYTHON_CONTEXT: &str = "python";

impl<'a> Processor<'a> {
    pub fn new(options: &'a Options,
               env: &'a Environment,
               package_prefix: Option<ast::Package>)
               -> Processor<'a> {
        Processor {
            options: options,
            env: env,
            package_prefix: package_prefix,
            to_lower_snake: naming::SnakeCase::new().to_lower_snake(),
        }
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

    fn build_constructor(&self, fields: &Vec<ast::Field>) -> MethodSpec {
        let mut constructor = MethodSpec::new("__init__");
        constructor.push_argument(python_stmt!["self"]);

        for field in fields {
            constructor.push_argument(python_stmt![&field.name]);
            constructor.push(python_stmt!["self.", &field.name, " = ", &field.name]);
        }

        constructor
    }

    fn process_type<L>(&self,
                       package: &ast::Package,
                       ty: &ast::TypeDecl,
                       listeners: &L)
                       -> Result<ClassSpec>
        where L: Listeners
    {
        let mut class = ClassSpec::new(&ty.name);
        Ok(class)
    }

    fn build_getters(&self, fields: &Vec<ast::Field>) -> Result<Vec<MethodSpec>> {
        let mut result = Vec::new();

        for field in fields {
            let name = self.to_lower_snake.convert(&field.name);
            let getter_name = format!("get_{}", name);
            let mut method_spec = MethodSpec::new(&getter_name);
            method_spec.push_argument(python_stmt!["self"]);
            method_spec.push(python_stmt!["return self.", name]);
            result.push(method_spec);
        }

        Ok(result)
    }

    fn process_message<L>(&self,
                          package: &ast::Package,
                          message: &ast::MessageDecl,
                          listeners: &L)
                          -> Result<ClassSpec>
        where L: Listeners
    {
        let mut class = ClassSpec::new(&message.name);

        let mut fields = Vec::new();

        for member in &message.members {
            if let ast::MessageMember::Field(ref field, _) = *member {
                fields.push(field.clone());
                continue;
            }
        }

        let constructor = self.build_constructor(&fields);
        class.push(&constructor);

        for getter in self.build_getters(&fields)? {
            class.push(getter);
        }

        for member in &message.members {
            if let ast::MessageMember::Code(ref context, ref content, _) = *member {
                if context == PYTHON_CONTEXT {
                    class.push(ElementSpec::Literal(content.clone()));
                }
            }
        }

        Ok(class)
    }

    fn process_interface<L>(&self,
                            package: &ast::Package,
                            interface: &ast::InterfaceDecl,
                            listeners: &L)
                            -> Result<ClassSpec>
        where L: Listeners
    {
        let mut interface_spec = ClassSpec::new(&interface.name);

        let mut interface_fields: Vec<ast::Field> = Vec::new();

        for member in &interface.members {
            if let ast::InterfaceMember::Field(ref field, _) = *member {
                interface_fields.push(field.clone());
            }
        }

        for (_, ref sub_type) in &interface.sub_types {
            let mut class = ClassSpec::new(&sub_type.name);
            let mut fields = interface_fields.clone();

            for member in &sub_type.members {
                if let ast::SubTypeMember::Field(ref field) = *member {
                    fields.push(field.clone());
                }
            }

            let constructor = self.build_constructor(&fields);
            class.push(&constructor);

            for getter in self.build_getters(&fields)? {
                class.push(&getter);
            }

            interface_spec.push(&class);
        }

        Ok(interface_spec)
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

        let mut files = HashMap::new();

        // Process all types discovered so far.
        for (&(ref package, _), decl) in &self.env.types {
            let class_spec = match *decl {
                ast::Decl::Interface(ref interface) => {
                    self.process_interface(package, interface, listeners)
                }
                ast::Decl::Message(ref message) => {
                    self.process_message(package, message, listeners)
                }
                ast::Decl::Type(ref ty) => self.process_type(package, ty, listeners),
            }?;

            match files.entry(package) {
                Entry::Vacant(entry) => {
                    let mut file_spec = FileSpec::new();
                    file_spec.push(class_spec);
                    entry.insert(file_spec);
                }
                Entry::Occupied(entry) => {
                    entry.into_mut().push(class_spec);
                }
            }
        }

        for (package, file_spec) in files.into_iter() {
            let mut full_path = self.java_package(package)
                .parts
                .iter()
                .fold(root_dir.clone(), |current, next| current.join(next));

            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }

            full_path.set_extension("py");

            debug!("Writing: {}", full_path.display());

            let out = file_spec.format();
            let mut f = File::create(full_path)?;
            let bytes = out.into_bytes();

            f.write_all(&bytes)?;
            f.flush()?;
        }

        Ok(())
    }
}
