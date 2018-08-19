#[macro_use]
extern crate log;
#[allow(unused)]
#[macro_use]
extern crate reproto_backend as backend;
extern crate reproto_core as core;
#[macro_use]
extern crate reproto_manifest as manifest;
extern crate reproto_trans as trans;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate linked_hash_map;
extern crate reproto_naming as naming;
extern crate serde_yaml as yaml;
extern crate toml;

const OPENAPI_VERSION: &str = "3.0.0";

use core::errors::*;
use core::flavored::{
    RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody, RpTupleBody, RpType, RpTypeBody,
    RpVersionedPackage,
};
use core::{CoreFlavor, Handle, Loc, RelativePath, RelativePathBuf, RpHttpMethod, Version};
use linked_hash_map::LinkedHashMap;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use std::any::Any;
use std::cell::RefCell;
use std::collections::{hash_map, BTreeMap, HashMap, HashSet, VecDeque};
use std::path::Path;
use trans::{Environment, Translated};

#[derive(Clone, Copy, Default, Debug)]
pub struct OpenApiLang;

impl Lang for OpenApiLang {
    lang_base!(OpenApiModule, compile);
}

#[derive(Debug)]
pub enum OpenApiModule {}

impl TryFromToml for OpenApiModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

struct Ref(String);

impl<'a> From<Ref> for Schema<'a> {
    fn from(reference: Ref) -> Self {
        Schema {
            reference: Some(reference.0),
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
struct SchemaBoolean {}

impl<'a> From<SchemaBoolean> for Schema<'a> {
    fn from(_: SchemaBoolean) -> Self {
        Schema {
            ty: Some("boolean"),
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
struct SchemaAny;

impl<'a> From<SchemaAny> for Schema<'a> {
    fn from(_: SchemaAny) -> Self {
        Schema::default()
    }
}

#[derive(Debug, Default)]
struct Integer {
    format: Option<Format>,
}

impl<'a> From<Integer> for Schema<'a> {
    fn from(integer: Integer) -> Self {
        Schema {
            ty: Some("integer"),
            format: integer.format,
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
struct SchemaString<'a> {
    enum_: Vec<&'a str>,
    format: Option<Format>,
}

impl<'a> From<SchemaString<'a>> for Schema<'a> {
    fn from(string: SchemaString<'a>) -> Self {
        Schema {
            ty: Some("string"),
            enum_: string.enum_,
            format: string.format,
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Object<'a> {
    required: Vec<&'a str>,
    properties: LinkedHashMap<&'a str, Schema<'a>>,
    additional_properties: Option<Box<Schema<'a>>>,
    title: Option<&'a str>,
}

impl<'a> From<Object<'a>> for Schema<'a> {
    fn from(object: Object<'a>) -> Self {
        Schema {
            ty: Some("object"),
            string_required: object.required,
            string_properties: object.properties,
            additional_properties: object.additional_properties,
            title: object.title,
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
struct SchemaArray<'a> {
    items: Option<Box<Schema<'a>>>,
    format: Option<Format>,
    required: Vec<usize>,
    /// For tuples, map each position to a type.
    properties: BTreeMap<usize, Schema<'a>>,
}

impl<'a> From<SchemaArray<'a>> for Schema<'a> {
    fn from(array: SchemaArray<'a>) -> Self {
        Schema {
            ty: Some("array"),
            items: array.items,
            format: array.format,
            usize_required: array.required,
            usize_properties: array.properties,
            ..Schema::default()
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
struct Info<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a Version>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
enum ParameterIn {
    #[serde(rename = "path")]
    Path,
}

impl Default for ParameterIn {
    fn default() -> Self {
        ParameterIn::Path
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
struct Parameter<'a> {
    name: &'a str,
    required: bool,
    in_: ParameterIn,
}

#[derive(Debug, Serialize)]
enum Format {
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "int64")]
    Int64,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "tuple")]
    Tuple,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Serialize)]
struct Schema<'a> {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    reference: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    ty: Option<&'static str>,

    /// How arrays specify inner item type.
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Box<Schema<'a>>>,

    /// Available enumerations of a string.
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    enum_: Vec<&'a str>,

    /// Format acts as extra specification of the type when needed.
    /// Also extensible.
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Format>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    one_of: Vec<Schema<'a>>,

    #[serde(rename = "required", skip_serializing_if = "Vec::is_empty")]
    string_required: Vec<&'a str>,

    #[serde(
        rename = "properties",
        skip_serializing_if = "LinkedHashMap::is_empty"
    )]
    string_properties: LinkedHashMap<&'a str, Schema<'a>>,

    #[serde(rename = "required", skip_serializing_if = "Vec::is_empty")]
    usize_required: Vec<usize>,

    #[serde(
        rename = "properties",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    usize_properties: BTreeMap<usize, Schema<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<Box<Schema<'a>>>,
}

struct OneOf<'a>(Vec<Schema<'a>>);

impl<'a> From<OneOf<'a>> for Schema<'a> {
    fn from(value: OneOf<'a>) -> Self {
        Schema {
            one_of: value.0,
            ..Schema::default()
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
struct Content<'a> {
    schema: Schema<'a>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
struct Response<'a> {
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    content: LinkedHashMap<String, Content<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
struct Method<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    parameters: Vec<Parameter<'a>>,
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    responses: LinkedHashMap<String, Response<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
struct SpecPath<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<Method<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Serialize)]
struct Components<'a> {
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    schemas: LinkedHashMap<String, Schema<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
struct Spec<'a> {
    openapi: &'static str,
    info: Info<'a>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    servers: Vec<&'a str>,
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    paths: LinkedHashMap<String, SpecPath<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Components<'a>>,
}

fn compile(handle: &Handle, env: Environment<CoreFlavor>, _manifest: Manifest) -> Result<()> {
    let env = env.translate_default()?;
    let compiler = Compiler::new(handle, env);
    compiler.compile()
}

struct Compiler<'handle> {
    upper_camel: naming::ToUpperCamel,
    handle: &'handle Handle,
    env: Translated<CoreFlavor>,
    any_type: RpName,
}

impl<'handle> Compiler<'handle> {
    pub fn new(handle: &'handle Handle, env: Translated<CoreFlavor>) -> Self {
        Compiler {
            upper_camel: naming::to_upper_camel(),
            handle,
            env,
            any_type: RpName::new(None, RpVersionedPackage::empty(), vec!["Any".to_string()]),
        }
    }

    fn compile(self) -> Result<()> {
        let root = RelativePathBuf::from(".");

        for (package, file) in self.env.for_each_file() {
            let mut dir = package
                .package
                .parts()
                .fold(root.clone(), |path, part| path.join(part));

            for d in file.for_each_decl() {
                // Use services as entrypoints.
                let service = match *d {
                    core::RpDecl::Service(ref service) => service,
                    _ => continue,
                };

                let mut builder = SpecBuilder {
                    upper_camel: &self.upper_camel,
                    handle: self.handle,
                    env: &self.env,
                    allocated_names: RefCell::new(HashMap::new()),
                    all_names: RefCell::new(HashSet::new()),
                    name_counters: RefCell::new(HashMap::new()),
                    any_type: &self.any_type,
                };

                let (spec, path) = builder.build(&dir, package, service)?;

                debug!("+file: {}", path.display());
                writeln!(self.handle.create(&path)?, "{}", yaml::to_string(&spec)?)?;
            }
        }

        Ok(())
    }
}

/// Queued up things that will be processed.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Queued<'a> {
    Named(&'a RpName),
    Any,
}

/// Keeps track of important and temporary state for a single Spec.
///
/// Allocates names that are conflict free.
struct SpecBuilder<'builder> {
    upper_camel: &'builder naming::ToUpperCamel,
    handle: &'builder Handle,
    env: &'builder Translated<CoreFlavor>,
    /// Names and what local names they are associated with.
    allocated_names: RefCell<HashMap<RpName, String>>,
    /// All allocated names to see if we conflict with an existing name.
    all_names: RefCell<HashSet<String>>,
    /// In case we can't generate a conflict-free name fall back to assigning an incrementing
    /// counter.
    name_counters: RefCell<HashMap<String, usize>>,
    /// Special any type that needs to be constructed.
    any_type: &'builder RpName,
}

impl<'builder> SpecBuilder<'builder> {
    /// Process a service into a Spec.
    fn build(
        self,
        dir: &RelativePath,
        package: &'builder RpVersionedPackage,
        service: &'builder RpServiceBody,
    ) -> Result<(Spec<'builder>, RelativePathBuf)> {
        let mut queue = VecDeque::new();

        let ident = if let Some(version) = package.version.as_ref() {
            format!("{}-{}.yaml", service.ident, version)
        } else {
            format!("{}.yaml", service.ident)
        };

        let path = dir.join(ident);

        let mut spec = Spec {
            openapi: OPENAPI_VERSION,
            info: Info::default(),
            servers: Vec::new(),
            paths: LinkedHashMap::new(),
            components: None,
        };

        if let Some(version) = package.version.as_ref() {
            spec.info.version = Some(version);
        }

        if let Some(ref url) = service.http.url {
            spec.servers.push(url);
        }

        // NB: we need to group each path.
        for e in &service.endpoints {
            let path = match e.http.path {
                Some(ref path) => path,
                None => continue,
            };

            let method = match e.http.method {
                Some(ref method) => *method,
                // TODO: handle during into_model transformation.
                None => RpHttpMethod::Get,
            };

            let mut p = spec
                .paths
                .entry(path.to_string())
                .or_insert_with(SpecPath::default);

            let method = match method {
                RpHttpMethod::Get => &mut p.get,
                RpHttpMethod::Head => &mut p.head,
                RpHttpMethod::Post => &mut p.post,
                RpHttpMethod::Put => &mut p.put,
                RpHttpMethod::Delete => &mut p.delete,
                m => return Err(format!("method `{:?}` is not supported", m).into()),
            };

            let method = method.get_or_insert_with(Method::default);
            method.operation_id = Some(e.safe_ident());

            if !e.comment.is_empty() {
                method.description = Some(e.comment.join("\n"));
            }

            if let Some(returns) = e.response.as_ref() {
                let mut response = Response::default();

                let schema = self.type_to_schema(&mut queue, returns.ty())?;

                let content = Content { schema };

                response
                    .content
                    .insert("application/json".to_string(), content);
                method.responses.insert("200".to_string(), response);

                if let core::RpType::Name { ref name } = *returns.ty() {
                    queue.push_back(Queued::Named(Loc::borrow(name)));
                }
            } else {
                // empty 200 by default
                let mut response = Response::default();
                method.responses.insert("200".to_string(), response);
            }
        }

        self.process_components(queue, &mut spec)?;

        if let Some(parent) = path.parent() {
            if !self.handle.is_dir(parent) {
                debug!("+dir: {}", parent.display());
                self.handle.create_dir_all(parent)?;
            }
        }

        Ok((spec, path))
    }

    fn process_components(
        &self,
        mut queue: VecDeque<Queued<'builder>>,
        spec: &mut Spec<'builder>,
    ) -> Result<()> {
        // components that have been processed.
        let mut processed = HashSet::new();

        while let Some(item) = queue.pop_front() {
            if !processed.insert(item) {
                continue;
            }

            let (ref_, schema) = match item {
                Queued::Named(name) => {
                    let ref_ = self.name_to_ref(name)?;
                    let decl = self.env.lookup_decl(name)?;

                    let schema = match *decl {
                        core::RpDecl::Type(ref body) => {
                            self.decl_type_to_schema(&mut queue, body)?
                        }
                        core::RpDecl::Interface(ref body) => {
                            self.decl_interface_to_schema(&mut queue, body)?
                        }
                        core::RpDecl::Enum(ref body) => self.decl_enum_to_schema(body)?,
                        core::RpDecl::Tuple(ref body) => {
                            self.decl_tuple_to_schema(&mut queue, body)?
                        }
                        _ => {
                            continue;
                        }
                    };

                    (ref_, schema)
                }
                Queued::Any => {
                    let ref_ = self.name_to_ref(self.any_type)?;
                    (ref_, Schema::from(SchemaAny))
                }
            };

            let mut components = spec.components.get_or_insert_with(Components::default);
            components.schemas.insert(ref_, schema);
        }

        Ok(())
    }

    /// Convert a declaration into a set of properties.
    fn decl_type_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        body: &'builder RpTypeBody,
    ) -> Result<Schema<'builder>> {
        let mut object = Object::default();
        self.populate_properties(queue, &mut object, body.fields())?;
        Ok(Schema::from(object))
    }

    /// Convert a declaration into a set of properties.
    fn decl_interface_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        body: &'builder RpInterfaceBody,
    ) -> Result<Schema<'builder>> {
        let mut fields = Vec::new();
        fields.extend(body.fields());

        match body.sub_type_strategy {
            core::RpSubTypeStrategy::Untagged => {
                let mut one_of = Vec::new();

                for sub_type in &body.sub_types {
                    let mut fields = fields.clone();
                    fields.extend(sub_type.fields());

                    let mut object = Object::default();
                    self.populate_properties(queue, &mut object, fields)?;
                    one_of.push(Schema::from(object));
                }

                Ok(Schema::from(OneOf(one_of)))
            }
            _ => {
                return Err("unsupported sub-type strategy".into());
            }
        }
    }

    /// Convert a declaration into a tuple.
    fn decl_tuple_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        body: &'builder RpTupleBody,
    ) -> Result<Schema<'builder>> {
        let mut array = SchemaArray::default();
        array.format = Some(Format::Tuple);

        for (index, field) in body.fields().enumerate() {
            let schema = self.type_to_schema(queue, field.ty())?;
            array.properties.insert(index, schema);
            array.required.push(index);

            // reference to external type, so add to queue.
            if let core::RpType::Name { ref name } = *field.ty() {
                queue.push_back(Queued::Named(name));
            }
        }

        Ok(Schema::from(array))
    }

    /// Convert a declaration into a set of properties.
    fn decl_enum_to_schema(&self, body: &'builder RpEnumBody) -> Result<Schema<'builder>> {
        let out = match body.variants {
            core::RpVariants::String { ref variants } => {
                let mut string = SchemaString::default();

                for v in variants {
                    string.enum_.push(v.value.as_str());
                }

                Schema::from(string)
            }
            // TODO: are numeric variants supported?
            core::RpVariants::Number { .. } => Schema::from(SchemaString::default()),
        };

        Ok(out)
    }

    /// Allocate a conflict-free name.
    fn allocate_name(&self, name: &RpName) -> Result<String> {
        use naming::Naming;

        let mut all_names = self
            .all_names
            .try_borrow_mut()
            .map_err(|_| "no mutable access")?;
        let concat_ident = name.path.join("");

        // local ident is sufficient
        if all_names.insert(concat_ident.to_string()) {
            return Ok(concat_ident);
        }

        // add concat package to concat identifier.
        let concat_package = name
            .package
            .package
            .parts()
            .map(|p| self.upper_camel.convert(p))
            .collect::<Vec<String>>()
            .join("");

        let test2 = format!("{}{}", concat_package, concat_ident);

        if all_names.insert(test2.clone()) {
            return Ok(test2);
        }

        // add version, if available
        let base = if let Some(version) = name.package.version.as_ref() {
            let mut parts = Vec::new();
            parts.push(format!("V{}", version.major));
            parts.push(version.minor.to_string());
            parts.push(version.patch.to_string());

            let version_concat = parts.join("");
            format!("{}{}{}", concat_package, version_concat, concat_ident)
        } else {
            format!("{}{}", concat_package, concat_ident)
        };

        let mut name_counters = self
            .name_counters
            .try_borrow_mut()
            .map_err(|_| "no mutable access")?;

        let c = match name_counters.entry(base.clone()) {
            hash_map::Entry::Occupied(mut e) => {
                let c = *e.get();
                *e.get_mut() += 1;
                c
            }
            hash_map::Entry::Vacant(e) => *e.insert(0usize),
        };

        let test4 = format!("{}{}", base, c);

        if all_names.insert(test4.clone()) {
            return Ok(test4);
        }

        Err(format!("cannot allocate conflict-free name for: {}", name).into())
    }

    /// Convert a name into a conflict-free reference.
    fn name_to_ref(&self, name: &RpName) -> Result<String> {
        let mut allocated_names = self
            .allocated_names
            .try_borrow_mut()
            .map_err(|_| "no mutable access")?;

        match allocated_names.entry(name.clone()) {
            hash_map::Entry::Vacant(e) => {
                let name = self.allocate_name(&name)?;
                e.insert(name.clone());
                Ok(name)
            }
            hash_map::Entry::Occupied(e) => Ok(e.get().to_string()),
        }
    }

    /// Convert the core type into a schema element.
    fn type_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        ty: &'builder RpType,
    ) -> Result<Schema<'builder>> {
        use core::RpType::*;

        let out = match *ty {
            Name { ref name } => {
                let ref_ = self.name_to_ref(name)?;
                Schema::from(Ref(format!("#/components/schemas/{}", ref_)))
            }
            // NB: only string keys are supported right now.
            Map { ref value, .. } => {
                let mut object = Object::default();
                object.additional_properties = Some(Box::new(self.type_to_schema(queue, value)?));
                Schema::from(object)
            }
            Array { ref inner } => {
                let mut array = SchemaArray::default();
                array.items = Some(Box::new(self.type_to_schema(queue, inner)?));
                Schema::from(array)
            }
            String => Schema::from(SchemaString::default()),
            Signed { size: 32 } | Unsigned { size: 32 } => Schema::from(Integer {
                format: Some(Format::Int32),
            }),
            Signed { size: 64 } | Unsigned { size: 64 } => Schema::from(Integer {
                format: Some(Format::Int64),
            }),
            Float => Schema::from(Integer {
                format: Some(Format::Float),
            }),
            Double => Schema::from(Integer {
                format: Some(Format::Double),
            }),
            Boolean => Schema::from(SchemaBoolean::default()),
            DateTime => {
                let mut string = SchemaString::default();
                string.format = Some(Format::DateTime);
                Schema::from(string)
            }
            Bytes => {
                let mut string = SchemaString::default();
                string.format = Some(Format::Byte);
                Schema::from(string)
            }
            Any => {
                queue.push_back(Queued::Any);
                let ref_ = self.name_to_ref(&self.any_type)?;
                Schema::from(Ref(format!("#/components/schemas/{}", ref_)))
            }
            ref ty => return Err(format!("unsupported type: {}", ty).into()),
        };

        Ok(out)
    }

    /// Populate properties on the given Object and collect additional types to process.
    fn populate_properties(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        object: &mut Object<'builder>,
        fields: impl IntoIterator<Item = &'builder Loc<RpField>>,
    ) -> Result<()> {
        for field in fields {
            let mut schema = self.type_to_schema(queue, field.ty())?;
            object.properties.insert(field.safe_ident(), schema);

            if field.is_required() {
                object.required.push(field.safe_ident());
            }

            if field.name() != field.safe_ident() {
                schema.title = Some(field.safe_ident());
            }

            // reference to external type, so add to queue.
            if let core::RpType::Name { ref name } = *field.ty() {
                queue.push_back(Queued::Named(name));
            }
        }

        Ok(())
    }
}
