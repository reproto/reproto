use environment::Environment;
use options::Options;
use parser::ast;
use std::fs::File;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use naming::{self, FromNaming};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[macro_use]
use codegen::python::*;

use errors::*;

const TYPE: &str = "type";
const INIT_PY: &str = "__init__.py";
const EXT: &str = "py";

pub trait Listeners {}

#[derive(Clone)]
pub struct Field {
    pub modifier: ast::Modifier,
    pub ty: ast::Type,
    pub name: String,
    pub ident: String,
}

impl Field {
    pub fn new(modifier: ast::Modifier, ty: ast::Type, name: String, ident: String) -> Field {
        Field {
            modifier: modifier,
            ty: ty,
            name: name,
            ident: ident,
        }
    }
}

pub struct Processor<'a> {
    options: &'a Options,
    env: &'a Environment,
    package_prefix: Option<ast::Package>,
    to_lower_snake: Box<naming::Naming>,
    staticmethod: BuiltInName,
    dict: BuiltInName,
    type_var: Variable,
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
            staticmethod: Name::built_in("staticmethod"),
            dict: Name::built_in("dict"),
            type_var: Variable::String(TYPE.to_owned()),
        }
    }

    /// Build a function that raises an exception if the given value `stmt` is None.
    fn raise_if_none(&self, stmt: &Statement, field: &Field) -> Elements {
        let mut raise_if_none = Elements::new();
        let required_error = Variable::String(format!("{}: is a required field", field.name));

        raise_if_none.push(python_stmt!["if ", &stmt, " is None:"]);
        raise_if_none.push_nested(python_stmt!["raise Exception(", required_error, ")"]);

        raise_if_none
    }

    fn encode_method<E>(&self,
                        package: &ast::Package,
                        fields: &Vec<Field>,
                        builder: &BuiltInName,
                        extra: E)
                        -> Result<MethodSpec>
        where E: FnOnce(&mut Elements) -> ()
    {
        let mut encode = MethodSpec::new("encode");
        encode.push_argument(python_stmt!["self"]);

        let mut encode_body = Elements::new();

        encode_body.push(python_stmt!["data = ", builder, "()"]);

        extra(&mut encode_body);

        for field in fields {
            let var_string = Variable::String(field.name.to_owned());
            let field_stmt = python_stmt!["self.", &field.ident];
            let value_stmt = self.encode(package, &field.ty, &field_stmt)?;

            match field.modifier {
                ast::Modifier::Optional => {
                    let mut check_if_none = Elements::new();

                    check_if_none.push(python_stmt!["if ", &field_stmt, " is not None:"]);

                    let stmt = python_stmt!["data[", var_string, "] = ", value_stmt];

                    check_if_none.push_nested(stmt);

                    encode_body.push(check_if_none);
                }
                _ => {
                    encode_body.push(self.raise_if_none(&field_stmt, field));

                    let stmt = python_stmt!["data[", var_string, "] = ", value_stmt];

                    encode_body.push(stmt);
                }
            }
        }

        encode_body.push(python_stmt!["return data"]);

        encode.push(encode_body.join(ElementSpec::Spacing));
        Ok(encode)
    }

    fn encode_tuple_method(&self,
                           package: &ast::Package,
                           fields: &Vec<Field>)
                           -> Result<MethodSpec> {
        let mut values = Statement::new();

        let mut encode = MethodSpec::new("encode");
        encode.push_argument(python_stmt!["self"]);

        let mut encode_body = Elements::new();

        for field in fields {
            let stmt = python_stmt!["self.", &field.name];
            encode_body.push(self.raise_if_none(&stmt, field));
            values.push(self.encode(package, &field.ty, stmt)?);
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
                        package: &ast::Package,
                        fields: &Vec<Field>,
                        class: &ClassSpec,
                        variable_fn: F)
                        -> Result<MethodSpec>
        where F: Fn(usize, &Field) -> Variable
    {
        let mut decode = MethodSpec::new("decode");
        decode.push_decorator(&self.staticmethod);
        decode.push_argument(python_stmt!["data"]);

        let mut decode_body = Elements::new();

        let mut arguments = Statement::new();

        for (i, field) in fields.iter().enumerate() {
            let var_name = format!("f_{}", field.ident);
            let var = variable_fn(i, field);

            let stmt = match field.modifier {
                ast::Modifier::Optional => {
                    let var_stmt = self.decode(package, &field.ty, &var_name)?;
                    self.optional_check(&var_name, &var, &var_stmt)
                }
                _ => {
                    let var_stmt = python_stmt!["data[", &var, "]"];
                    let var_stmt = self.decode(package, &field.ty, var_stmt)?;
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

    fn is_native(&self, ty: &ast::Type) -> bool {
        match *ty {
            ast::Type::I32 | ast::Type::U32 => true,
            ast::Type::I64 | ast::Type::U64 => true,
            ast::Type::Float | ast::Type::Double => true,
            ast::Type::String => true,
            ast::Type::Any => true,
            ast::Type::Tuple(ref elements) => elements.iter().all(|e| self.is_native(&e.ty)),
            ast::Type::Array(ref inner) => self.is_native(inner),
            _ => false,
        }
    }

    fn ident(&self, name: &str) -> String {
        if let Some(ref id_converter) = self.options.id_converter {
            id_converter.convert(name)
        } else {
            name.to_owned()
        }
    }

    fn custom_name(&self, package: &ast::Package, custom: &str) -> Name {
        let package = self.package(package);
        let key = &(package.clone(), custom.to_owned());
        let _ = self.env.types.get(key);
        Name::local(&custom).as_name()
    }

    fn used_name(&self, package: &ast::Package, used: &str, custom: &str) -> Result<Name> {
        let package = self.env.lookup_used(package, used)?;
        let package = self.package(package);
        let package = package.parts.join(".");
        Ok(Name::imported_alias(&package, &custom, used).as_name())
    }

    fn encode<S>(&self, package: &ast::Package, ty: &ast::Type, value_stmt: S) -> Result<Statement>
        where S: AsStatement
    {
        let value_stmt = value_stmt.as_statement();

        // TODO: do not skip conversion if strict type checking is enabled
        if self.is_native(ty) {
            return Ok(value_stmt);
        }

        let value_stmt = match *ty {
            ast::Type::I32 | ast::Type::U32 => value_stmt,
            ast::Type::I64 | ast::Type::U64 => value_stmt,
            ast::Type::Float | ast::Type::Double => value_stmt,
            ast::Type::String => value_stmt,
            ast::Type::Any => value_stmt,
            ast::Type::Custom(ref _custom) => python_stmt![value_stmt, ".encode()"],
            ast::Type::UsedType(ref _used, ref _custom) => python_stmt![value_stmt, ".encode()"],
            ast::Type::Array(ref inner) => {
                let v = python_stmt!["v"];
                let inner = self.encode(package, inner, v)?;
                python_stmt!["map(lambda v: ", inner, ", ", value_stmt, ")"]
            }
            _ => value_stmt,
        };

        Ok(value_stmt)
    }

    fn decode<S>(&self, package: &ast::Package, ty: &ast::Type, value_stmt: S) -> Result<Statement>
        where S: AsStatement
    {
        let value_stmt = value_stmt.as_statement();

        // TODO: do not skip conversion if strict type checking is enabled
        if self.is_native(ty) {
            return Ok(value_stmt);
        }

        let value_stmt = match *ty {
            ast::Type::I32 | ast::Type::U32 => value_stmt,
            ast::Type::I64 | ast::Type::U64 => value_stmt,
            ast::Type::Float | ast::Type::Double => value_stmt,
            ast::Type::String => value_stmt,
            ast::Type::Any => value_stmt,
            ast::Type::Custom(ref custom) => {
                let name = self.custom_name(package, custom);
                python_stmt![name, ".decode(", value_stmt, ")"]
            }
            ast::Type::UsedType(ref used, ref custom) => {
                let name = self.used_name(package, used, custom)?;
                python_stmt![name, ".decode(", value_stmt, ")"]
            }
            ast::Type::Array(ref inner) => {
                let inner = self.decode(package, inner, python_stmt!["v"])?;
                python_stmt!["map(lambda v: ", inner, ", ", value_stmt, ")"]
            }
            _ => value_stmt,
        };

        Ok(value_stmt)
    }


    /// Build the java package of a given package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    fn package(&self, package: &ast::Package) -> ast::Package {
        self.package_prefix
            .clone()
            .map(|prefix| prefix.join(package))
            .unwrap_or_else(|| package.clone())
    }

    fn build_constructor(&self, fields: &Vec<Field>) -> MethodSpec {
        let mut constructor = MethodSpec::new("__init__");
        constructor.push_argument(python_stmt!["self"]);

        for field in fields {
            constructor.push_argument(python_stmt![&field.ident]);
            constructor.push(python_stmt!["self.", &field.ident, " = ", &field.ident]);
        }

        constructor
    }

    fn process_type<L>(&self,
                       package: &ast::Package,
                       ty: &ast::TypeDecl,
                       _listeners: &L)
                       -> Result<ClassSpec>
        where L: Listeners
    {
        match ty.value {
            ast::Type::Tuple(ref elements) => {
                let mut class = ClassSpec::new(&ty.name);
                let mut fields: Vec<Field> = Vec::new();

                for (index, element) in elements.iter().enumerate() {
                    let index_name = match index {
                        0 => "first".to_owned(),
                        1 => "second".to_owned(),
                        2 => "third".to_owned(),
                        n => format!("field{}", n),
                    };

                    let name = element.name.clone().unwrap_or(index_name);
                    let ident = self.ident(&name);
                    fields.push(Field::new(ast::Modifier::Required, element.ty.clone(), name, ident));
                }

                class.push(self.build_constructor(&fields));

                // TODO: make configurable
                if false {
                    for getter in self.build_getters(&fields)? {
                        class.push(&getter);
                    }
                }

                self.tuple_added(package, &fields, &mut class)?;
                Ok(class)
            }
            _ => Err(format!("unsupported type: {:?}", ty).into()),
        }
    }

    fn build_getters(&self, fields: &Vec<Field>) -> Result<Vec<MethodSpec>> {
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
                          _listeners: &L)
                          -> Result<ClassSpec>
        where L: Listeners
    {
        let mut class = ClassSpec::new(&message.name);
        let mut fields = Vec::new();

        for member in &message.members {
            if let ast::MessageMember::Field(ref field, _) = *member {
                let ident = self.ident(&field.name);

                fields.push(Field::new(field.modifier.clone(),
                                       field.ty.clone(),
                                       field.name.clone(),
                                       ident));

                continue;
            }
        }

        let constructor = self.build_constructor(&fields);
        class.push(&constructor);

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class.push(getter);
            }
        }

        for member in &message.members {
            if let ast::MessageMember::Code(ref context, ref content, _) = *member {
                if context == PYTHON_CONTEXT {
                    class.push(ElementSpec::Literal(content.clone()));
                }

                continue;
            }
        }

        let decode = self.decode_method(package,
                           &fields,
                           &class,
                           |_, field| Variable::String(field.name.to_owned()))?;

        class.push(decode);

        let encode = self.encode_method(package, &fields, &self.dict, |_| {})?;

        class.push(encode);

        Ok(class)
    }

    fn process_interface<L>(&self,
                            package: &ast::Package,
                            interface: &ast::InterfaceDecl,
                            _listeners: &L)
                            -> Result<Vec<ClassSpec>>
        where L: Listeners
    {
        let mut classes = Vec::new();

        let mut interface_spec = ClassSpec::new(&interface.name);

        interface_spec.push(self.interface_decode_method(interface)?);

        let mut interface_fields: Vec<Field> = Vec::new();

        for member in &interface.members {
            if let ast::InterfaceMember::Field(ref field, _) = *member {
                let ident = self.ident(&field.name);

                interface_fields.push(Field::new(field.modifier.clone(),
                                                 field.ty.clone(),
                                                 field.name.clone(),
                                                 ident));

                continue;
            }

            if let ast::InterfaceMember::Code(ref context, ref content, _) = *member {
                if context == PYTHON_CONTEXT {
                    interface_spec.push(ElementSpec::Literal(content.clone()));
                }

                continue;
            }
        }

        classes.push(interface_spec);

        for (_, ref sub_type) in &interface.sub_types {
            let mut class = ClassSpec::new(&sub_type.name);
            class.extends(Name::local(&interface.name));

            let name = sub_type.options
                .lookup_string_nth("name", 0)
                .map(Clone::clone)
                .unwrap_or_else(|| interface.name.clone());

            class.push(python_stmt!["TYPE = ", Variable::String(name.clone())]);

            let mut fields = interface_fields.clone();

            for member in &sub_type.members {
                if let ast::SubTypeMember::Field(ref field) = *member {
                    let ident = self.ident(&field.name);

                    fields.push(Field::new(field.modifier.clone(),
                                           field.ty.clone(),
                                           field.name.clone(),
                                           ident));

                    continue;
                }
            }

            let constructor = self.build_constructor(&fields);
            class.push(&constructor);

            // TODO: make configurable
            if false {
                for getter in self.build_getters(&fields)? {
                    class.push(&getter);
                }
            }

            for member in &sub_type.members {
                if let ast::SubTypeMember::Code(ref context, ref content, _) = *member {
                    if context == PYTHON_CONTEXT {
                        class.push(ElementSpec::Literal(content.clone()));
                    }

                    continue;
                }
            }

            let decode = self.decode_method(package,
                               &fields,
                               &class,
                               |_, field| Variable::String(field.name.to_owned()))?;

            class.push(decode);

            let type_stmt =
                python_stmt!["data[", &self.type_var, "] = ", Variable::String(name.clone())];

            let encode = self.encode_method(package, &fields, &self.dict, move |elements| {
                    elements.push(type_stmt);
                })?;

            class.push(encode);

            classes.push(class);
        }

        Ok(classes)
    }

    fn populate_files<L>(&self, listeners: &L) -> Result<HashMap<&ast::Package, FileSpec>>
        where L: Listeners
    {
        let mut files = HashMap::new();

        // Process all types discovered so far.
        for (&(ref package, _), decl) in &self.env.types {
            let class_specs = match *decl {
                ast::Decl::Interface(ref interface) => {
                    self.process_interface(package, interface, listeners)?
                }
                ast::Decl::Message(ref message) => {
                    vec![self.process_message(package, message, listeners)?]
                }
                ast::Decl::Type(ref ty) => vec![self.process_type(package, ty, listeners)?],
            };

            match files.entry(package) {
                Entry::Vacant(entry) => {
                    let mut file_spec = FileSpec::new();

                    for class_spec in class_specs {
                        file_spec.push(class_spec);
                    }

                    entry.insert(file_spec);
                }
                Entry::Occupied(entry) => {
                    let mut file_spec = entry.into_mut();

                    for class_spec in class_specs {
                        file_spec.push(class_spec);
                    }
                }
            }
        }

        Ok(files)
    }

    fn setup_module_path(&self, root_dir: &PathBuf, package: &ast::Package) -> Result<PathBuf> {
        let package = self.package(package);

        let mut full_path = root_dir.to_owned();
        let mut iter = package.parts.iter().peekable();

        while let Some(part) = iter.next() {
            full_path = full_path.join(part);

            if iter.peek().is_none() {
                continue;
            }

            let init_path = full_path.join(INIT_PY);

            if !init_path.is_file() {
                if !full_path.is_dir() {
                    debug!("+dir: {}", full_path.display());
                    fs::create_dir_all(&full_path)?;
                }

                debug!("+init: {}", init_path.display());
                File::create(init_path)?;
            }
        }

        // path to final file
        full_path.set_extension(EXT);
        Ok(full_path)
    }

    fn write_files(&self, files: HashMap<&ast::Package, FileSpec>) -> Result<()> {
        let root_dir = &self.options.out_path;

        for (package, file_spec) in files {
            let full_path = self.setup_module_path(root_dir, package)?;

            debug!("+module: {}", full_path.display());

            let out = file_spec.format();
            let mut f = File::create(full_path)?;
            let bytes = out.into_bytes();

            f.write_all(&bytes)?;
            f.flush()?;
        }

        Ok(())
    }

    fn tuple_added(&self,
                   package: &ast::Package,
                   fields: &Vec<Field>,
                   class: &mut ClassSpec)
                   -> Result<()> {

        let decode = self.decode_method(package,
                           fields,
                           class,
                           |i, _| Variable::Literal(i.to_string()))?;

        let encode = self.encode_tuple_method(package, fields)?;

        class.push(decode);
        class.push(encode);
        Ok(())
    }

    fn interface_decode_method(&self, interface: &ast::InterfaceDecl) -> Result<MethodSpec> {
        let mut decode = MethodSpec::new("decode");
        decode.push_decorator(&self.staticmethod);
        decode.push_argument(python_stmt!["data"]);

        let mut decode_body = Elements::new();

        let type_field = Variable::Literal("f_type".to_owned());

        decode_body.push(python_stmt![&type_field, " = data[", &self.type_var, "]"]);

        for (_, ref sub_type) in &interface.sub_types {
            for name in sub_type.options.lookup_string("name") {
                let type_name = Name::local(&sub_type.name).as_name();

                let mut check = Elements::new();

                check.push(python_stmt!["if ",
                                        &type_field,
                                        " == ",
                                        Variable::String(name.to_owned()),
                                        ":"]);
                check.push_nested(python_stmt!["return ", type_name, ".decode(data)"]);

                decode_body.push(check);
            }
        }

        decode_body.push(python_stmt!["raise Exception(",
                                      Variable::String("bad type".to_owned()),
                                      " + ",
                                      &type_field,
                                      ")"]);

        decode.push(decode_body.join(ElementSpec::Spacing));

        Ok(decode)
    }

    pub fn process<L>(&self, listeners: &L) -> Result<()>
        where L: Listeners
    {
        let files = self.populate_files(listeners)?;
        self.write_files(files)
    }
}
