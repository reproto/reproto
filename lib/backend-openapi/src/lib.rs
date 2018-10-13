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
extern crate serde_json as json;
extern crate serde_yaml as yaml;
extern crate toml;

mod spec;

const OPENAPI_VERSION: &str = "3.0.0";

/// A number rule to set up an enum for a given numeric type.
macro_rules! number_rule {
    ($variants:ident, $name:ident, $convert:ident) => {{
        let mut __number = spec::$name::default();

        for n in $variants {
            let n = n.value.$convert().ok_or_else(|| "not a legal number")?;
            __number.enum_.push(n);
        }

        spec::Schema::from(__number)
    }};
}

use self::spec::*;
use core::errors::*;
use core::flavored::{
    RpChannel, RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody, RpTupleBody, RpType,
    RpTypeBody, RpVersionedPackage,
};
use core::{
    CoreFlavor, Handle, Loc, RelativePath, RelativePathBuf, RpHttpMethod, RpNumberKind, Span,
};
use linked_hash_map::LinkedHashMap;
use manifest::{checked_modules, Lang, Manifest, NoModule, TryFromToml};
use std::any::Any;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap, HashSet, VecDeque};
use std::path::Path;
use trans::{Session, Translated};

#[derive(Clone, Copy, Default, Debug)]
pub struct OpenApiLang;

impl Lang for OpenApiLang {
    lang_base!(OpenApiModule, compile);
}

#[derive(Debug)]
pub enum OpenApiModule {
    Json,
}

impl TryFromToml for OpenApiModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::OpenApiModule::*;

        let result = match id {
            "json" => Json,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::OpenApiModule::*;

        let result = match id {
            "json" => Json,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

fn compile(handle: &Handle, env: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let env = env.translate_default()?;

    let modules = checked_modules(manifest.modules)?;

    let mut compiler = Compiler::new(handle, env);
    compiler.load_options(modules)?;
    compiler.compile()
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Yaml,
    Json,
}

impl OutputFormat {
    /// Get the file extension to use.
    fn ext(&self) -> &'static str {
        use self::OutputFormat::*;

        match *self {
            Yaml => "yaml",
            Json => "json",
        }
    }
}

struct Compiler<'handle> {
    upper_camel: naming::ToUpperCamel,
    handle: &'handle Handle,
    env: Translated<CoreFlavor>,
    any_type: RpName,
    output_format: OutputFormat,
}

impl<'handle> Compiler<'handle> {
    pub fn new(handle: &'handle Handle, env: Translated<CoreFlavor>) -> Self {
        Compiler {
            upper_camel: naming::to_upper_camel(),
            handle,
            env,
            any_type: RpName::new(
                None,
                Loc::new(RpVersionedPackage::empty(), Span::empty()),
                vec!["Any".to_string()],
            ),
            output_format: OutputFormat::Yaml,
        }
    }

    /// Load options from the given modules.
    fn load_options(&mut self, modules: Vec<OpenApiModule>) -> Result<()> {
        use self::OpenApiModule::*;

        for module in &modules {
            match *module {
                Json => {
                    self.output_format = OutputFormat::Json;
                }
            }
        }

        Ok(())
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
                    output_format: self.output_format,
                };

                let (spec, path) = builder.build(&dir, package, service)?;

                debug!("+file: {}", path.display());

                let out = self.handle.create(&path)?;

                match self.output_format {
                    OutputFormat::Yaml => yaml::to_writer(out, &spec)?,
                    OutputFormat::Json => json::to_writer_pretty(out, &spec)?,
                }
            }
        }

        Ok(())
    }
}

/// Queued up things that will be processed.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Queued<'a> {
    /// Create an entity for the given sub-type.
    TaggedSubType(&'a str, &'a RpName, usize),
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
    /// Format to write output as.
    output_format: OutputFormat,
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

        let ext = self.output_format.ext();

        let ident = if let Some(version) = package.version.as_ref() {
            format!("{}-{}.{}", service.ident, version, ext)
        } else {
            format!("{}.{}", service.ident, ext)
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
            spec.servers.push(Server { url });
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
                RpHttpMethod::Update => &mut p.update,
                RpHttpMethod::Patch => &mut p.patch,
            };

            let method = method.get_or_insert_with(Method::default);

            for v in path.vars() {
                let schema = self.type_to_schema(&mut queue, v.channel.ty())?;

                let mut param = spec::Parameter {
                    name: v.safe_ident(),
                    required: true,
                    in_: ParameterIn::Path,
                    description: None,
                    schema: schema,
                };

                method.parameters.push(param);
            }

            method.operation_id = Some(e.safe_ident());

            if !e.comment.is_empty() {
                method.description = Some(e.comment.join("\n"));
            }

            if let Some(req) = e.request.as_ref() {
                let mut request =
                    self.channel_to_content(&mut queue, core::RpAccept::Json, &req.channel)?;
                request.required = true;
                method.request_body = Some(request);
            }

            let response = if let Some(res) = e.response.as_ref() {
                self.channel_to_content(&mut queue, e.http.accept, res)?
            } else {
                // empty by default
                Payload::default()
            };

            method.responses.insert("200", response);
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

    /// Convert a channel into request/response payload.
    fn channel_to_content(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        accept: core::RpAccept,
        channel: &'builder RpChannel,
    ) -> Result<Payload<'builder>> {
        let schema = self.type_to_schema(queue, channel.ty())?;

        let content_type = match accept {
            core::RpAccept::Text => "text/plain",
            core::RpAccept::Json => "application/json",
        };

        if let core::RpType::Name { ref name } = *channel.ty() {
            queue.push_back(Queued::Named(Loc::borrow(name)));
        }

        let mut payload = Payload::default();
        payload.content.insert(content_type, Content { schema });
        Ok(payload)
    }

    /// Process a queue of components.
    fn process_components(
        &self,
        mut queue: VecDeque<Queued<'builder>>,
        spec: &mut Spec<'builder>,
    ) -> Result<()> {
        use self::Queued::*;

        // components that have been processed.
        let mut processed = HashSet::new();

        while let Some(item) = queue.pop_front() {
            if !processed.insert(item) {
                continue;
            }

            let (ref_, schema) = match item {
                // A named type that was referenced by another type.
                Named(name) => {
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
                // Sub-type being referenced needs a body created for it.
                TaggedSubType(tag, name, index) => {
                    self.process_tagged_sub_type(&mut queue, tag, name, index)?
                }
                Any => {
                    let ref_ = self.name_to_ref(self.any_type)?;
                    (ref_, spec::Schema::from(spec::SchemaAny))
                }
            };

            let mut components = spec.components.get_or_insert_with(Components::default);
            components.schemas.insert(ref_, schema);
        }

        Ok(())
    }

    /// Process a single sub-type, creating a component that can be referenced.
    fn process_tagged_sub_type(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        tag: &'builder str,
        name: &RpName,
        index: usize,
    ) -> Result<(String, spec::Schema<'builder>)> {
        let decl = self.env.lookup_decl(name)?;

        let (body, sub_type) = match *decl {
            core::RpDecl::Interface(ref body) => match body.sub_types.get(index) {
                Some(sub_type) => (body, sub_type),
                None => return Err("bad sub-type index".into()),
            },
            _ => return Err("name does not refer to an interface".into()),
        };

        let ref_ = self.name_to_ref(&sub_type.name)?;

        // add the discriminator field
        let schema = spec::Schema::from(spec::SchemaString::default());

        let mut object = spec::Object::default();

        object.required.push(tag);
        object.properties.insert(tag, schema);

        let mut fields = Vec::new();
        fields.extend(body.fields());

        let mut fields = fields.clone();
        fields.extend(sub_type.fields());

        if !sub_type.comment.is_empty() {
            object.description = Some(sub_type.comment.join("\n"));
        }

        self.populate_properties(queue, &mut object, fields)?;
        Ok((ref_, spec::Schema::from(object)))
    }

    /// Convert a declaration into a set of properties.
    fn decl_type_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        body: &'builder RpTypeBody,
    ) -> Result<spec::Schema<'builder>> {
        let mut object = spec::Object::default();

        if !body.comment.is_empty() {
            object.description = Some(body.comment.join("\n"));
        }

        self.populate_properties(queue, &mut object, body.fields())?;
        Ok(spec::Schema::from(object))
    }

    /// Convert a declaration into a set of properties.
    fn decl_interface_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        body: &'builder RpInterfaceBody,
    ) -> Result<spec::Schema<'builder>> {
        let mut schema = spec::Schema::default();

        if !body.comment.is_empty() {
            schema.description = Some(body.comment.join("\n"));
        }

        match body.sub_type_strategy {
            core::RpSubTypeStrategy::Untagged => {
                let mut fields = Vec::new();
                fields.extend(body.fields());

                for sub_type in &body.sub_types {
                    let mut fields = fields.clone();
                    fields.extend(sub_type.fields());

                    let mut object = spec::Object::default();

                    if !sub_type.comment.is_empty() {
                        object.description = Some(sub_type.comment.join("\n"));
                    }

                    self.populate_properties(queue, &mut object, fields)?;
                    schema.one_of.push(spec::Schema::from(object));
                }
            }
            core::RpSubTypeStrategy::Tagged { ref tag } => {
                let mut discriminator = spec::Discriminator::default();

                discriminator.property_name = Some(tag);

                for (index, sub_type) in body.sub_types.iter().enumerate() {
                    let ref_ = self.name_to_ref(&sub_type.name)?;
                    let ref_ = format!("#/components/schemas/{}", ref_);

                    schema
                        .one_of
                        .push(spec::Schema::from(spec::Ref(ref_.to_string())));
                    queue.push_back(Queued::TaggedSubType(tag, &body.name, index));

                    discriminator.mapping.insert(sub_type.name(), ref_);
                }

                schema.discriminator = Some(discriminator);
            }
        }

        Ok(schema)
    }

    /// Convert a declaration into a tuple.
    fn decl_tuple_to_schema(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        body: &'builder RpTupleBody,
    ) -> Result<spec::Schema<'builder>> {
        let mut array = spec::SchemaArray::default();
        array.format = Some(spec::Format::Tuple);

        for (index, field) in body.fields().enumerate() {
            let schema = self.type_to_schema(queue, field.ty())?;
            array.properties.insert(index, schema);
            array.required.push(index);

            // reference to external type, so add to queue.
            if let core::RpType::Name { ref name } = *field.ty() {
                queue.push_back(Queued::Named(name));
            }
        }

        Ok(spec::Schema::from(array))
    }

    /// Convert a declaration into a set of properties.
    fn decl_enum_to_schema(&self, body: &'builder RpEnumBody) -> Result<spec::Schema<'builder>> {
        let out = match body.variants {
            core::RpVariants::String { ref variants } => {
                let mut string = spec::SchemaString::default();

                for v in variants {
                    string.enum_.push(v.value.as_str());
                }

                spec::Schema::from(string)
            }
            // TODO: are numeric variants supported?
            core::RpVariants::Number { ref variants } => match *body.enum_type {
                core::RpEnumType::Number(ref number) => match number.kind {
                    RpNumberKind::U32 => number_rule!(variants, U32, to_u32),
                    RpNumberKind::U64 => number_rule!(variants, U64, to_u64),
                    RpNumberKind::I32 => number_rule!(variants, I32, to_i32),
                    RpNumberKind::I64 => number_rule!(variants, I64, to_i64),
                },
                _ => return Err("unexpected enum type".into()),
            },
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
    ) -> Result<spec::Schema<'builder>> {
        use core::RpType::*;

        let out = match *ty {
            Argument { .. } => {
                return Err("OpenAPI: generic arguments are not supported".into());
            }
            Name { ref name } => {
                let ref_ = self.name_to_ref(name)?;
                spec::Schema::from(Ref(format!("#/components/schemas/{}", ref_)))
            }
            // NB: only string keys are supported right now.
            Map { ref value, .. } => {
                let mut object = spec::Object::default();
                object.additional_properties = Some(Box::new(self.type_to_schema(queue, value)?));
                spec::Schema::from(object)
            }
            Array { ref inner } => {
                let mut array = spec::SchemaArray::default();
                array.items = Some(Box::new(self.type_to_schema(queue, inner)?));
                spec::Schema::from(array)
            }
            String(..) => spec::Schema::from(spec::SchemaString::default()),
            Number(ref number) => match number.kind {
                RpNumberKind::I32 => spec::Schema::from(spec::I32::default()),
                RpNumberKind::I64 => spec::Schema::from(spec::I64::default()),
                RpNumberKind::U32 => spec::Schema::from(spec::U32::default()),
                RpNumberKind::U64 => spec::Schema::from(spec::U64::default()),
            },
            Float => spec::Schema::from(spec::Float::default()),
            Double => spec::Schema::from(spec::Double::default()),
            Boolean => spec::Schema::from(spec::SchemaBoolean::default()),
            DateTime => {
                let mut string = spec::SchemaString::default();
                string.format = Some(spec::Format::DateTime);
                spec::Schema::from(string)
            }
            Bytes => {
                let mut string = spec::SchemaString::default();
                string.format = Some(spec::Format::Byte);
                spec::Schema::from(string)
            }
            Any => {
                queue.push_back(Queued::Any);
                let ref_ = self.name_to_ref(&self.any_type)?;
                spec::Schema::from(Ref(format!("#/components/schemas/{}", ref_)))
            }
        };

        Ok(out)
    }

    /// Populate properties on the given Object and collect additional types to process.
    fn populate_properties(
        &self,
        queue: &mut VecDeque<Queued<'builder>>,
        object: &mut spec::Object<'builder>,
        fields: impl IntoIterator<Item = &'builder Loc<RpField>>,
    ) -> Result<()> {
        for field in fields {
            let mut schema = self.type_to_schema(queue, field.ty())?;

            if field.is_required() {
                object.required.push(field.safe_ident());
            }

            if field.name() != field.safe_ident() {
                schema.title = Some(field.safe_ident());
            }

            if !field.comment.is_empty() {
                schema.description = Some(field.comment.join("\n"));
            }

            object.properties.insert(field.safe_ident(), schema);

            // reference to external type, so add to queue.
            if let core::RpType::Name { ref name } = *field.ty() {
                queue.push_back(Queued::Named(name));
            }
        }

        Ok(())
    }
}
