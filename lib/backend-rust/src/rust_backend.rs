//! Backend for Rust

use super::RUST_CONTEXT;
use backend::{Code, PackageUtils};
use core::{ForEachLoc, Handle, Loc, RpEnumBody, RpEnumOrdinal, RpField, RpInterfaceBody, RpName,
           RpServiceBody, RpSubTypeStrategy, RpTupleBody, RpType, RpTypeBody};
use core::errors::*;
use genco::{Element, IntoTokens, Quoted, Rust, Tokens};
use genco::rust::{imported, imported_alias};
use listeners::Listeners;
use rust_compiler::RustCompiler;
use rust_file_spec::RustFileSpec;
use rust_options::RustOptions;
use std::rc::Rc;
use trans::Environment;

/// #[allow(non_camel_case_types)] attribute.
pub struct AllowNonCamelCaseTypes;

impl<'a> IntoTokens<'a, Rust<'a>> for AllowNonCamelCaseTypes {
    fn into_tokens(self) -> Tokens<'a, Rust<'a>> {
        "#[allow(non_camel_case_types)]".into()
    }
}

/// Serializer derives.
pub struct Derives;

impl<'a> IntoTokens<'a, Rust<'a>> for Derives {
    fn into_tokens(self) -> Tokens<'a, Rust<'a>> {
        "#[derive(Serialize, Deserialize, Debug)]".into()
    }
}

/// A serde rename annotation.
pub struct Rename<'a>(&'a str);

impl<'a> IntoTokens<'a, Rust<'a>> for Rename<'a> {
    fn into_tokens(self) -> Tokens<'a, Rust<'a>> {
        toks!["#[serde(rename = ", self.0.quoted(), ")]"]
    }
}

/// Tag attribute.
pub struct Tag<'a>(&'a str);

impl<'a> IntoTokens<'a, Rust<'a>> for Tag<'a> {
    fn into_tokens(self) -> Tokens<'a, Rust<'a>> {
        toks!["#[serde(tag = ", self.0.quoted(), ")]"]
    }
}

/// Documentation comments.
pub struct Comments<'el, S: 'el>(&'el [S]);

impl<'el, S: 'el + AsRef<str>> IntoTokens<'el, Rust<'el>> for Comments<'el, S> {
    fn into_tokens(self) -> Tokens<'el, Rust<'el>> {
        let mut t = Tokens::new();

        for c in self.0.iter() {
            t.push(toks!["/// ", c.as_ref()]);
        }

        t
    }
}

const TYPE_SEP: &'static str = "_";
const SCOPE_SEP: &'static str = "::";

pub struct RustBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
    hash_map: Rust<'static>,
    json_value: Rust<'static>,
    datetime: Option<Tokens<'static, Rust<'static>>>,
}

impl RustBackend {
    pub fn new(env: Environment, options: RustOptions, listeners: Box<Listeners>) -> RustBackend {
        RustBackend {
            env: env,
            listeners: listeners,
            hash_map: imported("std::collections", "HashMap"),
            json_value: imported_alias("serde_json", "Value", "json"),
            datetime: options.datetime.clone(),
        }
    }

    pub fn compiler<'el>(&'el self, handle: &'el Handle) -> Result<RustCompiler<'el>> {
        Ok(RustCompiler {
            handle: handle,
            backend: self,
        })
    }

    /// Build an implementation of the given name and body.
    fn build_impl<'el>(
        &self,
        name: Rc<String>,
        body: Tokens<'el, Rust<'el>>,
    ) -> Tokens<'el, Rust<'el>> {
        let mut out_impl = Tokens::new();

        out_impl.push(toks!["impl ", name.clone(), " {"]);
        out_impl.nested(body);
        out_impl.push("}");

        out_impl
    }

    /// Convert the type name
    ///
    /// Optionally also emit the necessary attributes to suppress warnings for bad naming
    /// conventions.
    fn convert_type_name(&self, name: &RpName) -> (Rc<String>, Tokens<'static, Rust<'static>>) {
        let attributes = if name.parts.len() > 1 {
            AllowNonCamelCaseTypes.into_tokens()
        } else {
            Tokens::new()
        };

        (Rc::new(name.join(TYPE_SEP)), attributes)
    }

    fn convert_type_id<'a>(&self, name: &'a RpName) -> Result<Element<'a, Rust<'a>>> {
        let registered = self.env.lookup(name)?;

        let local_name = registered.local_name(&name, |p| p.join(TYPE_SEP), |c| c.join(SCOPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let package_name = self.package(&name.package).parts.join("::");
            return Ok(
                imported_alias(package_name, local_name, prefix.as_str()).into(),
            );
        }

        Ok(local_name.into())
    }

    fn into_type<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Rust<'a>>> {
        let stmt = self.into_rust_type(&field.ty)?;

        if field.is_optional() {
            return Ok(toks!["Option<", stmt, ">"]);
        }

        Ok(stmt)
    }

    fn enum_value_fn<'a>(
        &self,
        name: Rc<String>,
        match_body: Tokens<'a, Rust<'a>>,
    ) -> Tokens<'a, Rust<'a>> {
        let mut value_fn = Tokens::new();
        let mut match_decl = Tokens::new();

        match_decl.push("match *self {");
        match_decl.nested(match_body);
        match_decl.push("}");

        value_fn.push("pub fn value(&self) -> &'static str {");
        value_fn.nested(toks!["use self::", name, "::*;"]);
        value_fn.nested(match_decl);
        value_fn.push("}");

        value_fn
    }

    fn datetime<'a>(&self, _ty: &RpType) -> Result<Tokens<'a, Rust<'a>>> {
        if let Some(ref datetime) = self.datetime {
            return Ok(datetime.clone().into());
        }

        Err(
            "Missing implementation for `datetime`, try: -m chrono".into(),
        )
    }

    pub fn into_rust_type<'a>(&self, ty: &'a RpType) -> Result<Tokens<'a, Rust<'a>>> {
        use self::RpType::*;

        let ty = match *ty {
            String => toks!["String"],
            DateTime => self.datetime(ty)?,
            Bytes => toks!["String"],
            Signed { size: 32 } => toks!["i32"],
            Signed { size: 64 } => toks!["i64"],
            Unsigned { size: 32 } => toks!["u32"],
            Unsigned { size: 64 } => toks!["u64"],
            Float => toks!["f32"],
            Double => toks!["f64"],
            Boolean => toks!["bool"],
            Array { ref inner } => {
                let argument = self.into_rust_type(inner)?;
                toks!["Vec<", argument, ">"]
            }
            Name { ref name } => toks![self.convert_type_id(name)?],
            Map { ref key, ref value } => {
                let key = self.into_rust_type(key)?;
                let value = self.into_rust_type(value)?;
                toks![self.hash_map.clone(), "<", key, ", ", value, ">"]
            }
            Any => toks![self.json_value.clone()],
            _ => return Err(format!("unsupported type: {}", ty).into()),
        };

        Ok(ty)
    }

    // Build the corresponding element out of a field declaration.
    fn field_element<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Rust<'a>>> {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let type_spec = self.into_type(field)?;

        if field.is_optional() {
            t.push(toks!["#[serde(skip_serializing_if=\"Option::is_none\")]"]);
        }

        if field.name() != ident {
            t.push(Rename(field.name()));
        }

        t.push(toks![ident, ": ", type_spec, ","]);

        Ok(t.into())
    }

    pub fn process_tuple<'a>(
        &self,
        out: &mut RustFileSpec<'a>,
        body: &'a RpTupleBody,
    ) -> Result<()> {
        let mut fields = Tokens::new();

        for field in &body.fields {
            fields.push(self.into_type(field)?);
        }

        let (name, attributes) = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push_unless_empty(attributes);
        t.push(Derives);
        t.push(toks!["struct ", name, "(", fields.join(", "), ");",]);

        out.0.push(t);
        Ok(())
    }

    pub fn process_enum<'a>(&self, out: &mut RustFileSpec<'a>, body: &'a RpEnumBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);

        // variant declarations
        let mut variants = Tokens::new();
        // body of value function
        let mut match_body = Tokens::new();

        body.variants.iter().for_each_loc(|variant| {
            let value = if let RpEnumOrdinal::String(ref s) = variant.ordinal {
                if s != variant.local_name.as_str() {
                    variants.push(Rename(s.as_str()));
                }

                s
            } else {
                variant.local_name.as_str()
            };

            match_body.push(toks![
                variant.local_name.as_str(),
                " => ",
                value.quoted(),
                ",",
            ]);

            variants.push_unless_empty(Comments(&variant.comment));
            variants.push(toks![variant.local_name.as_str(), ","]);
            Ok(()) as Result<()>
        })?;

        out.0.push({
            let mut t = Tokens::new();

            t.push_unless_empty(Comments(&body.comment));
            t.push_unless_empty(attributes);
            t.push(Derives);
            t.push(toks!["pub enum ", name.clone(), " {"]);
            t.nested(variants);
            t.push("}");

            t
        });

        out.0.push({
            let mut t = Tokens::new();

            t.push(toks!["impl ", name.clone(), " {"]);

            t.nested({
                let mut t = Tokens::new();
                t.push(self.enum_value_fn(name.clone(), match_body));
                t.push_unless_empty(Code(&body.codes, RUST_CONTEXT));
                t
            });

            t.push("}");

            t
        });

        Ok(())
    }

    pub fn process_type<'a>(&self, out: &mut RustFileSpec<'a>, body: &'a RpTypeBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);
        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push_unless_empty(attributes);
        t.push(Derives);
        t.push(toks!["pub struct ", name.clone(), " {"]);

        // fields
        t.nested({
            let mut t = Tokens::new();

            for field in &body.fields {
                t.push_unless_empty(Comments(&field.comment));
                t.push(Loc::take(Loc::and_then(
                    Loc::as_ref(field),
                    |f| self.field_element(f),
                )?));
            }

            t
        });

        t.push("}");

        out.0.push(t);

        // if custom code is present, punt it into an impl.
        let impl_body = Code(&body.codes, RUST_CONTEXT).into_tokens();

        if !impl_body.is_empty() {
            out.0.push(self.build_impl(name.clone(), impl_body));
        }

        Ok(())
    }

    pub fn process_interface<'a>(
        &self,
        out: &mut RustFileSpec<'a>,
        body: &'a RpInterfaceBody,
    ) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push_unless_empty(attributes);
        t.push(Derives);

        match body.sub_type_strategy {
            RpSubTypeStrategy::Tagged { ref tag, .. } => {
                t.push(Tag(tag.as_str()));
            }
        }

        t.push(toks!["pub enum ", name.clone(), " {"]);

        let sub_types = body.sub_types.values().map(AsRef::as_ref);

        sub_types.for_each_loc(|s| {
            t.nested({
                let mut t = Tokens::new();

                t.push_unless_empty(Comments(&s.comment));

                // TODO: clone should not be needed
                if let Some(ref name) = s.sub_type_name {
                    if name.as_str() != s.local_name.as_str() {
                        t.push(Rename(name));
                    }
                }

                t.push(toks![s.local_name.as_str(), " {"]);

                for field in body.fields.iter().chain(s.fields.iter()) {
                    t.nested(self.field_element(field)?);
                }

                t.push("},");

                t
            });

            Ok(()) as Result<()>
        })?;

        t.push("}");

        out.0.push(t);

        let impl_body = Code(&body.codes, RUST_CONTEXT).into_tokens();

        if !impl_body.is_empty() {
            out.0.push(self.build_impl(name.clone(), impl_body));
        }

        Ok(())
    }

    pub fn process_service<'a>(
        &self,
        out: &mut RustFileSpec<'a>,
        body: &'a RpServiceBody,
    ) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);
        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push_unless_empty(attributes);
        t.push(toks!["pub trait ", name.clone(), " {"]);

        let endpoints = body.endpoints.values().map(Loc::as_ref);

        endpoints.for_each_loc(|e| {
            t.nested({
                let mut t = Tokens::new();
                t.push_unless_empty(Comments(&e.comment));
                t.push(toks!["fn ", e.safe_ident(), "();"]);
                t
            });

            Ok(()) as Result<()>
        })?;

        t.push("}");

        out.0.push(t);

        Ok(())
    }
}

impl PackageUtils for RustBackend {}
