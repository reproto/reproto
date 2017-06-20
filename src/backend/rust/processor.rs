use backend::*;
use backend::collecting::Collecting;
use backend::for_context::ForContext;
use backend::package_processor::PackageProcessor;
use codeviz::rust::*;
use core::*;
use errors::*;
use naming::{self, FromNaming};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

const EXT: &str = "rs";
const RUST_CONTEXT: &str = "rust";

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
}

impl ProcessorOptions {
    pub fn new() -> ProcessorOptions {
        ProcessorOptions {}
    }
}

pub struct Processor {
    env: Environment,
    out_path: PathBuf,
    id_converter: Option<Box<naming::Naming>>,
    package_prefix: Option<RpPackage>,
    listeners: Box<Listeners>,
    to_lower_snake: Box<naming::Naming>,
    hash_map: ImportedName,
    json_value: ImportedName,
}

impl Processor {
    pub fn new(_options: ProcessorOptions,
               env: Environment,
               out_path: PathBuf,
               id_converter: Option<Box<naming::Naming>>,
               package_prefix: Option<RpPackage>,
               listeners: Box<Listeners>)
               -> Processor {
        Processor {
            env: env,
            out_path: out_path,
            id_converter: id_converter,
            package_prefix: package_prefix,
            listeners: listeners,
            to_lower_snake: naming::SnakeCase::new().to_lower_snake(),
            hash_map: Name::imported("std::collections", "HashMap"),
            json_value: Name::imported_alias("serde_json", "Value", "json"),
        }
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
            .map_err(|e| Error::pos(e.description().to_owned(), pos.clone()))?;

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
                return Err(Error::pos(format!("unsupported type: {:?}", t), pos.clone()));
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

    fn write_mod_files(&self, files: &BTreeMap<&RpVersionedPackage, FileSpec>) -> Result<()> {
        let mut packages: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();

        for (key, _) in files {
            let mut current = self.out_path().to_owned();

            let mut it = self.package(key).parts.into_iter().peekable();

            while let Some(part) = it.next() {
                current = current.join(part);

                if let Some(next) = it.peek() {
                    let mut full_path = current.clone();

                    full_path = full_path.join("mod");
                    full_path.set_extension(self.ext());

                    packages.entry(full_path)
                        .or_insert_with(Vec::new)
                        .push(next.clone());
                }
            }
        }

        for (full_path, children) in packages {
            if let Some(parent) = full_path.parent() {
                if !parent.is_dir() {
                    debug!("+dir: {}", parent.display());
                    fs::create_dir_all(parent)?;
                }
            }

            if !full_path.is_file() {
                debug!("+mod: {}", full_path.display());
                let mut f = File::create(full_path)?;

                for child in children {
                    writeln!(f, "pub mod {};", child)?;
                }
            }
        }

        Ok(())
    }
}

impl Backend for Processor {
    fn process(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_mod_files(&files)?;
        self.write_files(files)
    }

    fn verify(&self) -> Result<Vec<Error>> {
        Ok(vec![])
    }
}

impl Collecting for FileSpec {
    type Processor = Processor;

    fn new() -> Self {
        FileSpec::new()
    }

    fn into_bytes(self, _: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();
        self.format(&mut out)?;
        Ok(out.into_bytes())
    }
}

impl PackageProcessor for Processor {
    type Out = FileSpec;

    fn ext(&self) -> &str {
        EXT
    }

    fn env(&self) -> &Environment {
        &self.env
    }

    fn package_prefix(&self) -> &Option<RpPackage> {
        &self.package_prefix
    }

    fn out_path(&self) -> &Path {
        &self.out_path
    }

    fn default_process(&self, _out: &mut Self::Out, _type_id: &RpTypeId, _: &RpPos) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self,
                     out: &mut Self::Out,
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

    fn process_enum(&self,
                    out: &mut Self::Out,
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

    fn process_type(&self,
                    out: &mut Self::Out,
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

    fn process_interface(&self,
                         out: &mut Self::Out,
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
