use options::Options;
use parser::ast;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::collections::HashMap;

#[macro_use]
use codegen::java::*;

use errors::*;

pub trait Listeners {
    fn class_added(&self, _class: &mut ClassSpec) -> Result<()> {
        Ok(())
    }
}

pub struct Processor {
    files: Vec<ast::File>,
    types: HashMap<(ast::Package, String), ast::Decl>,
    object: Type,
    list: Type,
    string: Type,
    integer: Type,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            files: Vec::new(),
            types: HashMap::new(),
            object: Type::new("java.lang", "Object"),
            list: Type::new("java.util", "List"),
            string: Type::new("java.lang", "String"),
            integer: Type::new("java.lang", "Integer"),
        }
    }

    fn convert_type(&self, package: &ast::Package, type_: &ast::Type) -> Result<TypeSpec> {
        let type_ = match *type_ {
            ast::Type::String => self.string.as_type_spec(),
            ast::Type::I32 => self.integer.as_type_spec(),
            ast::Type::Array(ref type_) => {
                let argument = self.convert_type(package, type_)?;
                self.list.with_arguments(vec![argument])
            }
            ast::Type::Custom(ref string) => {
                let key = &(package.clone(), string.clone());
                // let _type = self.types.get(key).ok_or(format!("No such type: {}", string))?;
                Type::new(&package.parts.join("."), string).as_type_spec()
            }
            _ => self.object.as_type_spec(),
        };

        Ok(type_)
    }

    fn build_constructor(&self, fields: &Vec<FieldSpec>) -> ConstructorSpec {
        let mut constructor = ConstructorSpec::new(mods![Modifier::Public]);

        for field in fields {
            let argument = ArgumentSpec::new(mods![Modifier::Final], &field.type_, &field.name);
            constructor.push_argument(&argument);
            constructor.push_statement(&stmt!["this.$L = $N", literal field.name.clone(), name argument]);
        }

        constructor
    }

    fn process_message<L>(&self,
                          file: &ast::File,
                          message: &ast::MessageDecl,
                          listeners: &L)
                          -> Result<FileSpec>
        where L: Listeners
    {
        let package_name = file.package.parts.join(".");

        let mut class = ClassSpec::new(mods![Modifier::Public], &message.name);

        let mut fields: Vec<FieldSpec> = Vec::new();

        for member in &message.members {
            if let ast::MessageMember::Field(ref field) = *member {
                fields.push(self.push_field(&file.package, &mut class, field)?);
            }
        }

        class.push_constructor(&self.build_constructor(&fields));

        listeners.class_added(&mut class)?;

        let mut file_spec = FileSpec::new(&package_name);
        file_spec.push_class(&class);

        Ok(file_spec)
    }

    fn process_interface<L>(&self,
                            file: &ast::File,
                            interface: &ast::InterfaceDecl,
                            listeners: &L)
                            -> Result<FileSpec>
        where L: Listeners
    {
        let package_name = file.package.parts.join(".");

        let mut interface_spec = InterfaceSpec::new(mods![Modifier::Public], &interface.name);

        for member in &interface.members {
            if let ast::InterfaceMember::SubType(ref sub_type) = *member {
                let mods = mods![Modifier::Public, Modifier::Static];
                let mut class = ClassSpec::new(mods, &sub_type.name);

                let mut fields: Vec<FieldSpec> = Vec::new();

                for m in &sub_type.members {
                    if let ast::SubTypeMember::Field(ref field) = *m {
                        fields.push(self.push_field(&file.package, &mut class, field)?);
                    }
                }

                class.push_constructor(&self.build_constructor(&fields));

                listeners.class_added(&mut class)?;

                interface_spec.push_class(&class);
            }
        }

        let mut file_spec = FileSpec::new(&package_name);
        file_spec.push_interface(&interface_spec);
        Ok(file_spec)
    }

    fn push_field(&self,
                  package: &ast::Package,
                  class: &mut ClassSpec,
                  field: &ast::Field)
                  -> Result<FieldSpec> {
        let field_type = self.convert_type(package, &field.type_)?;
        let mods = mods![Modifier::Private, Modifier::Final];
        let field = FieldSpec::new(mods, &field_type, &field.name);

        class.push_field(&field);

        Ok(field)
    }

    pub fn add_file(&mut self, file: ast::File) -> Result<()> {
        {
            let package = &file.package;

            for decl in &file.decls {
                let name = match *decl {
                    ast::Decl::Interface(ref interface) => Some(interface.name.clone()),
                    ast::Decl::Message(ref message) => Some(message.name.clone()),
                    _ => None,
                };

                if let Some(name) = name {
                    self.types.insert((package.clone(), name), decl.clone());
                }
            }
        }

        self.files.push(file);

        Ok(())
    }

    pub fn process<L>(&self, options: &Options, listeners: &L) -> Result<()>
        where L: Listeners
    {
        let root = options.out_path.clone();

        for file in &self.files {
            let out_dir =
                file.package.parts.iter().fold(root.clone(), |current, next| current.join(next));

            fs::create_dir_all(&out_dir)?;

            for decl in &file.decls {
                let res = match *decl {
                    ast::Decl::Interface(ref interface) => {
                        let full_path = out_dir.join(format!("{}.java", interface.name));
                        info!("Processing: {}", full_path.display());
                        Some((full_path, self.process_interface(file, interface, listeners)?))
                    }
                    ast::Decl::Message(ref message) => {
                        let full_path = out_dir.join(format!("{}.java", message.name));
                        info!("Processing: {}", full_path.display());
                        Some((full_path, self.process_message(file, message, listeners)?))
                    }
                    _ => None,
                };

                if let Some((full_path, file_spec)) = res {
                    info!("Writing: {}", full_path.display());

                    let out = file_spec.format()?;
                    let mut f = File::create(full_path)?;
                    let bytes = out.into_bytes();

                    f.write_all(&bytes)?;
                    f.flush()?;
                }
            }
        }

        Ok(())
    }
}
