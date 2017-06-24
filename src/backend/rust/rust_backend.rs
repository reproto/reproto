use backend::for_context::ForContext;
use naming::{self, FromNaming};
use std::rc::Rc;
use super::*;

pub struct RustBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
    id_converter: Option<Box<naming::Naming>>,
    to_lower_snake: Box<naming::Naming>,
    hash_map: ImportedName,
    json_value: ImportedName,
}

impl RustBackend {
    pub fn new(env: Environment,
               _: RustOptions,
               listeners: Box<Listeners>,
               id_converter: Option<Box<naming::Naming>>)
               -> RustBackend {
        RustBackend {
            env: env,
            listeners: listeners,
            id_converter: id_converter,
            to_lower_snake: naming::SnakeCase::new().to_lower_snake(),
            hash_map: Name::imported("std::collections", "HashMap"),
            json_value: Name::imported_alias("serde_json", "Value", "json"),
        }
    }

    pub fn compiler(&self, options: CompilerOptions) -> Result<RustCompiler> {
        Ok(RustCompiler {
            out_path: options.out_path,
            backend: self,
        })
    }

    pub fn verify(&self) -> Result<()> {
        Ok(())
    }

    fn ident(&self, name: &str) -> String {
        if let Some(ref id_converter) = self.id_converter {
            id_converter.convert(name)
        } else {
            name.to_owned()
        }
    }

    fn convert_type_id(&self, pos: &RpPos, type_id: &RpTypeId) -> Result<Name> {
        let (package, registered) = self.env
            .lookup(&type_id.package, &type_id.name)
            .map_err(|e| Error::pos(e.description().to_owned(), pos.into()))?;

        let name = registered.name().join(".");

        if let Some(ref prefix) = type_id.name.prefix {
            let package_name = self.package(package).parts.join("::");
            return Ok(Name::Imported(Name::imported_alias(&package_name, &name, prefix)));
        }

        Ok(Name::Local(Name::local(&name)))
    }

    fn into_type(&self, type_id: &RpTypeId, field: &RpLoc<RpField>) -> Result<Statement> {
        let stmt = self.into_rust_type(type_id, field.pos(), &field.ty)?;

        if field.is_optional() {
            return Ok(stmt!["Option<", stmt, ">"]);
        }

        Ok(stmt)
    }

    pub fn into_rust_type(&self,
                          type_id: &RpTypeId,
                          pos: &RpPos,
                          ty: &RpType)
                          -> Result<Statement> {
        let ty = match *ty {
            RpType::String => stmt!["String"],
            RpType::Signed { ref size } => {
                if size.map(|s| s <= 32usize).unwrap_or(true) {
                    stmt!["i32"]
                } else {
                    stmt!["i64"]
                }
            }
            RpType::Unsigned { ref size } => {
                if size.map(|s| s <= 32usize).unwrap_or(true) {
                    stmt!["u32"]
                } else {
                    stmt!["u64"]
                }
            }
            RpType::Float => stmt!["f32"],
            RpType::Double => stmt!["f64"],
            RpType::Boolean => stmt!["bool"],
            RpType::Array { ref inner } => {
                let argument = self.into_rust_type(type_id, pos, inner)?;
                stmt!["Vec<", argument, ">"]
            }
            RpType::Name { ref name } => {
                let type_id = type_id.with_name(name.clone());
                stmt![self.convert_type_id(pos, &type_id)?]
            }
            RpType::Map { ref key, ref value } => {
                let key = self.into_rust_type(type_id, pos, key)?;
                let value = self.into_rust_type(type_id, pos, value)?;
                stmt![&self.hash_map, "<", key, ", ", value, ">"]
            }
            RpType::Any => stmt![&self.json_value],
            ref t => {
                return Err(Error::pos(format!("unsupported type: {:?}", t), pos.into()));
            }
        };

        Ok(ty)
    }

    // Build the corresponding element out of a field declaration.
    fn field_element(&self, type_id: &RpTypeId, field: &RpLoc<RpField>) -> Result<Element> {
        let mut elements = Elements::new();

        let ident = self.ident(field.ident());
        let type_spec = self.into_type(type_id, field)?;

        if field.is_optional() {
            elements.push(stmt!["#[serde(skip_serializing_if=\"Option::is_none\")]"]);
        }

        if field.name() != ident {
            elements.push(stmt!["#[serde(rename = ", Variable::String(field.name().to_owned()), ")]"]);
        }

        elements.push(stmt![ident, ": ", type_spec, ","]);

        Ok(elements.into())
    }

    pub fn process_tuple(&self,
                         out: &mut FileSpec,
                         type_id: &RpTypeId,
                         _: &RpPos,
                         body: Rc<RpTupleBody>)
                         -> Result<()> {
        let mut fields = Statement::new();

        for field in &body.fields {
            fields.push(self.into_type(type_id, field)?);
        }

        let mut elements = Elements::new();
        elements.push("#[derive(Serialize, Deserialize, Debug)]");
        elements.push(stmt!["struct ", &body.name, "(", fields.join(", "), ");"]);

        out.push(elements);
        Ok(())
    }

    pub fn process_enum(&self,
                        out: &mut FileSpec,
                        _: &RpTypeId,
                        _: &RpPos,
                        body: Rc<RpEnumBody>)
                        -> Result<()> {
        let mut enum_spec = EnumSpec::new(&body.name);
        enum_spec.public();

        for code in body.codes.for_context(RUST_CONTEXT) {
            enum_spec.push(code.move_inner().lines);
        }

        out.push(enum_spec);
        Ok(())
    }

    pub fn process_type(&self,
                        out: &mut FileSpec,
                        type_id: &RpTypeId,
                        _: &RpPos,
                        body: Rc<RpTypeBody>)
                        -> Result<()> {
        let mut fields = Elements::new();

        for field in &body.fields {
            fields.push(self.field_element(type_id, field)?);
        }

        let mut struct_spec = StructSpec::new(&body.name);
        struct_spec.public();

        struct_spec.push_attribute("#[derive(Serialize, Deserialize, Debug)]");
        struct_spec.push(fields);

        for code in body.codes.for_context(RUST_CONTEXT) {
            struct_spec.push(code.move_inner().lines);
        }

        out.push(struct_spec);
        Ok(())
    }

    pub fn process_interface(&self,
                             out: &mut FileSpec,
                             type_id: &RpTypeId,
                             _: &RpPos,
                             body: Rc<RpInterfaceBody>)
                             -> Result<()> {
        let mut enum_spec = EnumSpec::new(&body.name);
        enum_spec.public();

        enum_spec.push_attribute("#[derive(Serialize, Deserialize, Debug)]");
        enum_spec.push_attribute("#[serde(tag = \"type\")]");

        for code in body.codes.for_context(RUST_CONTEXT) {
            enum_spec.push(code.move_inner().lines);
        }

        for (_, ref sub_type) in &body.sub_types {
            let mut elements = Elements::new();

            if let Some(name) = sub_type.names.first() {
                elements.push(stmt!["#[serde(rename = ",
                                    Variable::String((**name).to_owned()),
                                    ")]"]);
            }

            elements.push(stmt![&sub_type.name, " {"]);

            for field in body.fields.iter().chain(sub_type.fields.iter()) {
                elements.push_nested(self.field_element(type_id, field)?);
            }

            elements.push("},");

            enum_spec.push(elements);
        }

        out.push(enum_spec);

        Ok(())
    }
}

impl<'a> Collecting<'a> for FileSpec {
    type Processor = RustCompiler<'a>;

    fn new() -> Self {
        FileSpec::new()
    }

    fn into_bytes(self, _: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();
        self.format(&mut out)?;
        Ok(out.into_bytes())
    }
}

impl PackageUtils for RustBackend {}
