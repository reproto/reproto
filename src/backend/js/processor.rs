use backend::*;
use backend::errors::*;
use codeviz::js::*;
use naming::{self, FromNaming};
use options::Options;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use super::models::*;
use super::utils::*;

const TYPE: &str = "type";
const EXT: &str = "js";

fn field_ident(_i: usize, field: &JsField) -> Variable {
    string(&field.ident)
}

fn field_index(i: usize, _field: &JsField) -> Variable {
    Variable::Literal(i.to_string())
}

pub trait Listeners {
    fn configure(&self, _processor: &mut ProcessorOptions) -> Result<()> {
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
}

pub struct ProcessorOptions {
    parent: Options,
    pub build_getters: bool,
    pub build_constructor: bool,
}

impl ProcessorOptions {
    pub fn new(options: Options) -> ProcessorOptions {
        ProcessorOptions {
            parent: options,
            build_getters: false,
            build_constructor: true,
        }
    }
}

pub struct Processor {
    options: ProcessorOptions,
    env: Environment,
    package_prefix: Option<Package>,
    listeners: Box<Listeners>,
    to_lower_snake: Box<naming::Naming>,
    error: BuiltInName,
    type_var: Variable,
    members: Statement,
}

const JS_CONTEXT: &str = "js";

impl Processor {
    pub fn new(options: ProcessorOptions,
               env: Environment,
               package_prefix: Option<Package>,
               listeners: Box<Listeners>)
               -> Processor {
        Processor {
            options: options,
            env: env,
            package_prefix: package_prefix,
            listeners: listeners,
            to_lower_snake: naming::SnakeCase::new().to_lower_snake(),
            error: Name::built_in("Error"),
            type_var: string(TYPE),
            members: stmt!["__members__"],
        }
    }

    fn find_field<'a>(&self,
                      fields: &'a Vec<Token<JsField>>,
                      name: &str)
                      -> Option<(usize, &'a JsField)> {
        for (i, field) in fields.iter().enumerate() {
            if field.name == name {
                return Some((i, &field.inner));
            }
        }

        None
    }

    /// Build a function that throws an exception if the given value `stmt` is None.
    fn throw_if_null<S>(&self, stmt: S, field: &JsField) -> Elements
        where S: Into<Statement>
    {
        let required_error = string(format!("{}: is a required field", field.name));
        if_stmt!(is_not_defined(stmt),
                 stmt!["throw new", &self.error, "(", required_error, ");"])
    }

    fn convert_fields(&self, fields: &Vec<Token<Field>>) -> Vec<Token<JsField>> {
        fields.iter()
            .map(|f| {
                let ident = self.ident(&f.name);

                f.clone().map_inner(|o| {
                    JsField {
                        modifier: o.modifier,
                        ty: o.ty,
                        name: o.name,
                        ident: ident,
                    }
                })
            })
            .collect()
    }

    fn encode_method<E, B>(&self,
                           package: &Package,
                           fields: &Vec<Token<JsField>>,
                           builder: B,
                           extra: E)
                           -> Result<MethodSpec>
        where E: FnOnce(&mut Elements) -> (),
              B: Into<Variable>
    {
        let mut encode = MethodSpec::new("encode");

        let mut encode_body = Elements::new();

        encode_body.push(stmt!["let data = ", builder, ";"]);

        extra(&mut encode_body);

        for field in fields {
            let var_string = string(field.ident.to_owned());
            let field_stmt = stmt!["this.", &field.ident];
            let value_stmt = self.encode(package, &field.ty, &field_stmt)?;

            match field.modifier {
                Modifier::Optional => {
                    let stmt = if_stmt!(is_not_defined(field_stmt),
                                        stmt!["data[", var_string, "] = ", value_stmt, ";"]);
                    encode_body.push(stmt);
                }
                _ => {
                    encode_body.push(self.throw_if_null(field_stmt, field));
                    let stmt = stmt!["data[", var_string, "] = ", value_stmt, ";"];
                    encode_body.push(stmt);
                }
            }
        }

        encode_body.push(stmt!["return data"]);

        encode.push(encode_body.join(ElementSpec::Spacing));
        Ok(encode)
    }

    fn encode_tuple_method(&self,
                           package: &Package,
                           fields: &Vec<Token<JsField>>)
                           -> Result<MethodSpec> {
        let mut values = Statement::new();

        let mut encode = MethodSpec::new("encode");

        let mut encode_body = Elements::new();

        for field in fields {
            let stmt = stmt!["this.", &field.ident];
            encode_body.push(self.throw_if_null(&stmt, field));
            values.push(self.encode(package, &field.ty, stmt)?);
        }

        encode_body.push(stmt!["return [", values.join(", "), "];"]);
        encode.push(encode_body.join(ElementSpec::Spacing));
        Ok(encode)
    }

    fn encode_enum_method(&self, field: &JsField) -> Result<MethodSpec> {
        let mut encode = MethodSpec::new("encode");
        let mut encode_body = Elements::new();
        encode_body.push(stmt!["return self.", &field.ident]);
        encode.push(encode_body.join(ElementSpec::Spacing));
        Ok(encode)
    }

    fn decode_enum_method(&self, field: &JsField) -> Result<MethodSpec> {
        let mut decode = MethodSpec::with_static("decode");

        let data = stmt!["data"];
        let i = stmt!["i"];
        let member_name = stmt!["m"];
        let member = stmt!["member"];

        decode.push_argument(&data);

        let mut decode_body = Elements::new();

        let mut member_loop = Elements::new();

        let for_loop =
            stmt!["for (var ", &i, " = 0; i < this.", &self.members, ".length; ", &i, "++) {"];

        let name_assign =
            stmt!["var ", &member_name, " = this.", &self.members, ".length[", &i, "];"];

        member_loop.push(for_loop);
        member_loop.push_nested(name_assign);
        member_loop.push_nested(stmt!["var ", &member, " = this[", &member_name, "];"]);

        let cond = stmt![&member, ".", &field.ident, " === ", data];
        member_loop.push_nested(if_stmt!(cond, stmt!["return ", &member]));

        member_loop.push("}");

        let mismatch = string("data does not match enum".to_owned());
        let throw = stmt!["throw new ", &self.error, "(", mismatch, ")"];

        decode_body.push(member_loop);
        decode_body.push(throw);

        decode.push(decode_body.join(ElementSpec::Spacing));
        Ok(decode)
    }

    fn decode_method<F>(&self,
                        package: &Package,
                        fields: &Vec<Token<JsField>>,
                        class: &ClassSpec,
                        variable_fn: F)
                        -> Result<MethodSpec>
        where F: Fn(usize, &JsField) -> Variable
    {
        let mut decode = MethodSpec::new("static decode");
        decode.push_argument(stmt!["data"]);

        let mut decode_body = Elements::new();

        let mut arguments = Statement::new();

        for (i, field) in fields.iter().enumerate() {
            let var_name = format!("f_{}", field.ident);
            let var = variable_fn(i, field);

            let stmt: ElementSpec = match field.modifier {
                Modifier::Optional => {
                    let var_stmt = self.decode(&field.pos, package, &field.ty, &var_name)?;

                    let mut check = Elements::new();

                    check.push(stmt!["var ", &var_name, " = data[", &var, "];"]);
                    check.push(ElementSpec::Spacing);
                    check.push(if_stmt!(is_defined(stmt![&var_name]),
                                        stmt![&var_name, " = ", var_stmt, ";"],
                                        stmt![&var_name, " = null;"]));

                    check.into()
                }
                _ => {
                    let var_stmt = stmt!["data[", &var, "]"];
                    let var_stmt = self.decode(&field.pos, package, &field.ty, var_stmt)?;
                    stmt![&var_name, " = ", &var_stmt, ";"].into()
                }
            };

            decode_body.push(stmt);
            arguments.push(var_name);
        }

        let arguments = arguments.join(", ");
        decode_body.push(stmt!["return new ", &class.name, "(", arguments, ");"]);

        decode.push(decode_body.join(ElementSpec::Spacing));

        Ok(decode)
    }

    fn is_native(&self, ty: &Type) -> bool {
        match *ty {
            Type::Signed(_) |
            Type::Unsigned(_) => true,
            Type::Float | Type::Double => true,
            Type::String => true,
            Type::Any => true,
            Type::Boolean => true,
            Type::Array(ref inner) => self.is_native(inner),
            _ => false,
        }
    }

    fn ident(&self, name: &str) -> String {
        if let Some(ref id_converter) = self.options.parent.id_converter {
            id_converter.convert(name)
        } else {
            name.to_owned()
        }
    }

    fn custom_name(&self, package: &Package, custom: &str) -> Name {
        let package = self.package(package);
        let key = &(package.clone(), custom.to_owned());
        let _ = self.env.types.get(key);
        Name::local(&custom).into()
    }

    fn used_name(&self, pos: &Pos, package: &Package, used: &str, custom: &str) -> Result<Name> {
        let package = self.env.lookup_used(pos, package, used)?;
        let package = self.package(package);
        let package = package.parts.join(".");
        Ok(Name::imported_alias(&package, &custom, used).into())
    }

    fn encode<S>(&self, package: &Package, ty: &Type, value_stmt: S) -> Result<Statement>
        where S: Into<Statement>
    {
        let value_stmt = value_stmt.into();

        // TODO: do not skip conversion if strict type checking is enabled
        if self.is_native(ty) {
            return Ok(value_stmt);
        }

        let value_stmt = match *ty {
            Type::Signed(_) |
            Type::Unsigned(_) => value_stmt,
            Type::Float | Type::Double => value_stmt,
            Type::String => value_stmt,
            Type::Any => value_stmt,
            Type::Boolean => value_stmt,
            Type::Custom(ref _custom) => stmt![value_stmt, ".encode()"],
            Type::UsedType(ref _used, ref _custom) => stmt![value_stmt, ".encode()"],
            Type::Array(ref inner) => {
                let v = stmt!["v"];
                let inner = self.encode(package, inner, &v)?;
                stmt![value_stmt, ".map(function(", &v, ") { return ", inner, "; })"]
            }
            _ => value_stmt,
        };

        Ok(value_stmt)
    }

    fn decode<S>(&self, pos: &Pos, package: &Package, ty: &Type, value_stmt: S) -> Result<Statement>
        where S: Into<Statement>
    {
        let value_stmt = value_stmt.into();

        // TODO: do not skip conversion if strict type checking is enabled
        if self.is_native(ty) {
            return Ok(value_stmt);
        }

        let value_stmt = match *ty {
            Type::Signed(_) |
            Type::Unsigned(_) => value_stmt,
            Type::Float | Type::Double => value_stmt,
            Type::String => value_stmt,
            Type::Any => value_stmt,
            Type::Boolean => value_stmt,
            Type::Custom(ref custom) => {
                let name = self.custom_name(package, custom);
                stmt![name, ".decode(", value_stmt, ")"]
            }
            Type::UsedType(ref used, ref custom) => {
                let name = self.used_name(pos, package, used, custom)?;
                stmt![name, ".decode(", value_stmt, ")"]
            }
            Type::Array(ref inner) => {
                let inner = self.decode(pos, package, inner, stmt!["v"])?;
                stmt![value_stmt, ".map(function(v) { ", inner, "; })"]
            }
            _ => value_stmt,
        };

        Ok(value_stmt)
    }


    /// Build the java package of a given package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    fn package(&self, package: &Package) -> Package {
        self.package_prefix
            .clone()
            .map(|prefix| prefix.join(package))
            .unwrap_or_else(|| package.clone())
    }

    fn build_constructor(&self, fields: &Vec<Token<JsField>>) -> ConstructorSpec {
        let mut constructor = ConstructorSpec::new();
        let mut assignments = Elements::new();

        for field in fields {
            constructor.push_argument(stmt![&field.ident]);
            assignments.push(stmt!["this.", &field.ident, " = ", &field.ident, ";"]);
        }

        constructor.push(assignments);
        constructor
    }

    fn process_tuple(&self, package: &Package, ty: &TupleBody) -> Result<ClassSpec> {
        let mut class = ClassSpec::new(&ty.name);
        let mut fields: Vec<Token<JsField>> = Vec::new();

        for field in &ty.fields {
            let ident = self.ident(&field.name);

            fields.push(field.clone()
                .map_inner(|f| {
                    JsField {
                        modifier: Modifier::Required,
                        ty: f.ty,
                        name: f.name,
                        ident: ident,
                    }
                }));
        }

        for code in &ty.codes {
            if code.context == JS_CONTEXT {
                class.push(code.lines.clone());
            }
        }

        class.push(self.build_constructor(&fields));

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class.push(&getter);
            }
        }

        let decode = self.decode_method(package, &fields, &class, field_index)?;
        class.push(decode);

        let encode = self.encode_tuple_method(package, &fields)?;
        class.push(encode);

        Ok(class)
    }

    fn literal_value(&self, pos: &Pos, value: &Value, ty: &Type) -> Result<Variable> {
        match *ty {
            Type::Double |
            Type::Float |
            Type::Signed(_) |
            Type::Unsigned(_) |
            Type::Boolean => {
                if let Value::Boolean(ref boolean) = *value {
                    return Ok(Variable::Literal(boolean.to_string()));
                }

                if let Value::Number(ref number) = *value {
                    return Ok(Variable::Literal(number.to_string()));
                }
            }
            Type::String => {
                if let Value::String(ref s) = *value {
                    return Ok(string(s));
                }
            }
            _ => {}
        }

        Err(Error::pos(format!("{} cannot be applied to expected type {}", value, ty),
                       pos.clone()))
    }

    fn process_enum(&self, _package: &Package, body: &EnumBody) -> Result<ClassSpec> {
        let mut class = ClassSpec::new(&body.name);
        let mut fields: Vec<Token<JsField>> = Vec::new();

        for field in &body.fields {
            let ident = self.ident(&field.name);

            fields.push(field.clone()
                .map_inner(|f| {
                    JsField {
                        modifier: Modifier::Required,
                        ty: f.ty,
                        name: f.name,
                        ident: ident,
                    }
                }));
        }

        let mut members = Statement::new();
        let mut values = Elements::new();

        for value in &body.values {
            let mut value_arguments = Statement::new();

            for (value, field) in value.arguments.iter().zip(fields.iter()) {
                value_arguments.push(self.literal_value(&value.pos, value, &field.ty)?);
            }

            let arguments = stmt!["new ", &body.name, "(", value_arguments.join(", "), ")"];
            values.push(stmt!["static ", &value.name, " = ", arguments, ";"]);
            members.push(string(&value.name));
        }

        class.push(stmt!["static ", &self.members, " = [", members.join(", "), "]"]);
        class.push(values);

        for code in &body.codes {
            if code.context == JS_CONTEXT {
                class.push(code.lines.clone());
            }
        }

        class.push(self.build_constructor(&fields));

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class.push(&getter);
            }
        }

        // lookup serialized_as if specified.
        if let Some(ref s) = body.serialized_as {
            if let Some((_, ref field)) = self.find_field(&fields, &s.inner) {
                class.push(self.encode_enum_method(field)?);
                class.push(self.decode_enum_method(field)?);
            } else {
                return Err(Error::pos(format!("no field named: {}", s.inner), s.pos.clone()));
            }
        }

        Ok(class)
    }

    fn build_getters(&self, fields: &Vec<Token<JsField>>) -> Result<Vec<MethodSpec>> {
        let mut result = Vec::new();

        for field in fields {
            let name = self.to_lower_snake.convert(&field.ident);
            let getter_name = format!("get_{}", name);
            let mut method_spec = MethodSpec::new(&getter_name);
            method_spec.push(stmt!["return this.", name]);
            result.push(method_spec);
        }

        Ok(result)
    }

    fn process_type(&self, package: &Package, ty: &TypeBody) -> Result<ClassSpec> {
        let fields = self.convert_fields(&ty.fields);

        let mut class = ClassSpec::new(&ty.name);

        let constructor = self.build_constructor(&fields);
        class.push(&constructor);

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class.push(getter);
            }
        }

        for code in &ty.codes {
            if code.context == JS_CONTEXT {
                class.push(code.lines.clone());
            }
        }

        let decode = self.decode_method(package, &fields, &class, field_ident)?;
        class.push(decode);

        let encode = self.encode_method(package, &fields, "{}", |_| {})?;
        class.push(encode);

        Ok(class)
    }

    fn process_interface(&self,
                         package: &Package,
                         interface: &InterfaceBody)
                         -> Result<Vec<ClassSpec>> {
        let mut classes = Vec::new();

        let mut interface_spec = ClassSpec::new(&interface.name);

        interface_spec.push(self.interface_decode_method(interface)?);

        let interface_fields = self.convert_fields(&interface.fields);

        for code in &interface.codes {
            if code.context == JS_CONTEXT {
                interface_spec.push(code.lines.clone());
            }
        }

        classes.push(interface_spec);

        for (_, ref sub_type) in &interface.sub_types {
            let mut class = ClassSpec::new(&sub_type.name);

            let name: String = sub_type.names
                .iter()
                .map(|t| t.inner.to_owned())
                .nth(0)
                .unwrap_or_else(|| interface.name.clone());

            class.push(stmt!["static TYPE = ", string(name.clone()), ";"]);

            let mut fields = interface_fields.clone();
            fields.extend(self.convert_fields(&interface.fields));

            let constructor = self.build_constructor(&fields);
            class.push(&constructor);

            // TODO: make configurable
            if false {
                for getter in self.build_getters(&fields)? {
                    class.push(&getter);
                }
            }

            for code in &sub_type.codes {
                if code.context == JS_CONTEXT {
                    class.push(code.lines.clone());
                }
            }

            let decode = self.decode_method(package, &fields, &class, field_ident)?;

            class.push(decode);

            let type_stmt = stmt!["data[", &self.type_var, "] = ", string(name.clone()), ";"];

            let encode = self.encode_method(package, &fields, "{}", move |elements| {
                    elements.push(type_stmt);
                })?;

            class.push(encode);

            classes.push(class);
        }

        Ok(classes)
    }

    fn populate_files(&self) -> Result<HashMap<&Package, FileSpec>> {
        let mut files = HashMap::new();

        // Process all types discovered so far.
        for (&(ref package, _), decl) in &self.env.types {
            let class_specs: Vec<ClassSpec> = match decl.inner {
                Decl::Interface(ref body) => self.process_interface(package, body)?,
                Decl::Type(ref body) => vec![self.process_type(package, body)?],
                Decl::Tuple(ref body) => vec![self.process_tuple(package, body)?],
                Decl::Enum(ref body) => vec![self.process_enum(package, body)?],
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

    fn setup_module_path(&self, root_dir: &PathBuf, package: &Package) -> Result<PathBuf> {
        let package = self.package(package);

        let mut full_path = root_dir.to_owned();
        let mut iter = package.parts.iter().peekable();

        while let Some(part) = iter.next() {
            full_path = full_path.join(part);
        }

        if let Some(parent) = full_path.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent)?;
            }
        }

        // path to final file
        full_path.set_extension(EXT);
        Ok(full_path)
    }

    fn write_files(&self, files: HashMap<&Package, FileSpec>) -> Result<()> {
        let root_dir = &self.options.parent.out_path;

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

    fn interface_decode_method(&self, interface: &InterfaceBody) -> Result<MethodSpec> {
        let mut decode = MethodSpec::with_static("decode");
        decode.push_argument(stmt!["data"]);

        let mut decode_body = Elements::new();

        let type_field = Variable::Literal("f_type".to_owned());

        decode_body.push(stmt!["var ", &type_field, " = data[", &self.type_var, "]"]);

        for (_, ref sub_type) in &interface.sub_types {
            for name in &sub_type.names {
                let type_name: Variable = Name::local(&sub_type.name).into();
                let cond = stmt![&type_field, " === ", string(&name.inner)];
                decode_body.push(if_stmt!(cond, stmt!["return ", type_name, ".decode(data);"]));
            }
        }

        decode_body.push(stmt!["throw new ",
                               &self.error,
                               "(",
                               string("bad type"),
                               " + ",
                               &type_field,
                               ")"]);

        decode.push(decode_body.join(ElementSpec::Spacing));

        Ok(decode)
    }
}

impl Backend for Processor {
    fn process(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_files(files)
    }

    fn verify(&self) -> Result<Vec<Error>> {
        Ok(vec![])
    }
}
