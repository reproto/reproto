//! Backend for Rust

use super::RUST_CONTEXT;
use backend::{CompilerOptions, Environment, ForContext, FromNaming, Naming, PackageUtils,
              SnakeCase};
use backend::errors::*;
use core::{ForEachLoc, RpEnumBody, RpEnumOrdinal, RpField, RpInterfaceBody, RpName, RpTupleBody,
           RpType, RpTypeBody};
use genco::{Element, Quoted, Rust, Tokens};
use genco::rust::{imported_alias, imported_alias_ref, imported_ref};
use listeners::Listeners;
use rust_compiler::RustCompiler;
use rust_file_spec::RustFileSpec;
use rust_options::RustOptions;
use std::borrow::Cow;

const TYPE_SEP: &'static str = "_";
const SCOPE_SEP: &'static str = "::";

pub struct RustBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
    id_converter: Option<Box<Naming>>,
    to_lower_snake: Box<Naming>,
    hash_map: Rust<'static>,
    json_value: Rust<'static>,
}

impl RustBackend {
    pub fn new(
        env: Environment,
        _: RustOptions,
        listeners: Box<Listeners>,
        id_converter: Option<Box<Naming>>,
    ) -> RustBackend {
        RustBackend {
            env: env,
            listeners: listeners,
            id_converter: id_converter,
            to_lower_snake: SnakeCase::new().to_lower_snake(),
            hash_map: imported_ref("std::collections", "HashMap"),
            json_value: imported_alias_ref("serde_json", "Value", "json"),
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

    fn convert_type_name(&self, name: &RpName) -> String {
        name.join(TYPE_SEP)
    }

    fn convert_type_id<'a>(&self, name: &'a RpName) -> Result<Element<'a, Rust<'a>>> {
        let registered = self.env.lookup(name)?;

        let local_name = registered.local_name(&name, |p| p.join(TYPE_SEP), |c| c.join(SCOPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let package_name = self.package(&name.package).parts.join("::");
            return Ok(
                imported_alias(
                    Cow::Owned(package_name),
                    Cow::Owned(local_name),
                    Cow::Borrowed(prefix),
                ).into(),
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
        name: String,
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

    pub fn into_rust_type<'a>(&self, ty: &'a RpType) -> Result<Tokens<'a, Rust<'a>>> {
        use self::RpType::*;

        let ty = match *ty {
            String => toks!["String"],
            Bytes => toks!["String"],
            Signed { ref size } => {
                if size.map(|s| s <= 32usize).unwrap_or(true) {
                    toks!["i32"]
                } else {
                    toks!["i64"]
                }
            }
            Unsigned { ref size } => {
                if size.map(|s| s <= 32usize).unwrap_or(true) {
                    toks!["u32"]
                } else {
                    toks!["u64"]
                }
            }
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
        };

        Ok(ty)
    }

    // Build the corresponding element out of a field declaration.
    fn field_element<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Rust<'a>>> {
        let mut elements = Tokens::new();

        let ident = self.ident(field.ident());
        let type_spec = self.into_type(field)?;

        if field.is_optional() {
            elements.push(toks!["#[serde(skip_serializing_if=\"Option::is_none\")]"]);
        }

        if field.name() != ident {
            elements.push(toks!["#[serde(rename = ", field.name().quoted(), ")]"]);
        }

        elements.push(toks![ident, ": ", type_spec, ","]);

        Ok(elements.into())
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

        let name = self.convert_type_name(&body.name);

        let mut elements = Tokens::new();
        elements.push("#[derive(Serialize, Deserialize, Debug)]");
        elements.push(toks![
            "struct ",
            name,
            "(",
            fields.join(", "),
            ");",
        ]);

        out.0.push(elements);
        Ok(())
    }

    pub fn process_enum<'a>(&self, out: &mut RustFileSpec<'a>, body: &'a RpEnumBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);

        // variant declarations
        let mut variants = Tokens::new();
        // body of value function
        let mut match_body = Tokens::new();

        body.variants.for_each_loc(|variant| {
            let value = if let RpEnumOrdinal::String(ref s) = variant.ordinal {
                if s != variant.local_name.value() {
                    let rename = toks!["#[serde(rename = ", s.to_owned().quoted(), ")]"];
                    variants.push(rename);
                }

                s
            } else {
                &variant.local_name
            };

            // TODO: should not be necessary to clone since it derives from the body.
            match_body.push(toks![
                variant.local_name.value().as_str(),
                " => ",
                value.clone().quoted(),
                ",",
            ]);

            variants.push(toks![variant.local_name.value().as_str(), ","]);
            Ok(()) as Result<()>
        })?;

        let mut out_enum = Tokens::new();

        out_enum.push("#[derive(Serialize, Deserialize, Debug)]");
        out_enum.push(toks!["pub enum ", name.clone(), " {"]);
        out_enum.nested(variants);
        out_enum.push("}");

        let mut out_impl = Tokens::new();

        out_impl.push(toks!["impl ", name.clone(), " {"]);
        out_impl.nested(self.enum_value_fn(name, match_body));

        // code goes into impl
        for code in body.codes.for_context(RUST_CONTEXT) {
            for line in &code.lines {
                out_impl.nested(line.as_str());
            }
        }

        out_impl.push("}");

        out.0.push(out_enum);
        out.0.push(out_impl);
        Ok(())
    }

    pub fn process_type<'a>(&self, out: &mut RustFileSpec<'a>, body: &'a RpTypeBody) -> Result<()> {
        let mut fields = Tokens::new();

        for field in &body.fields {
            fields.push(self.field_element(field)?);
        }

        let name = self.convert_type_name(&body.name);
        let mut t = Tokens::new();

        t.push("#[derive(Serialize, Deserialize, Debug)]");
        t.push(toks!["pub struct ", name, " {"]);
        t.nested(fields);

        // TODO: clone should not be needed
        for code in body.codes.for_context(RUST_CONTEXT) {
            for line in &code.lines {
                t.nested(toks!(line.as_str()));
            }
        }

        t.push("}");

        out.0.push(t);
        Ok(())
    }

    pub fn process_interface<'a>(
        &self,
        out: &mut RustFileSpec<'a>,
        body: &'a RpInterfaceBody,
    ) -> Result<()> {
        let type_name = self.convert_type_name(&body.name);
        let mut t = Tokens::new();

        t.push("#[derive(Serialize, Deserialize, Debug)]");
        t.push("#[serde(tag = \"type\")]");
        t.push(toks!["pub enum ", type_name, " {"]);

        for code in body.codes.for_context(RUST_CONTEXT) {
            for line in &code.lines {
                t.nested(toks!(line.to_string()));
            }
        }

        let sub_types = body.sub_types.values().map(AsRef::as_ref);

        sub_types.for_each_loc(|sub_type| {
            let mut elements = Tokens::new();

            // TODO: clone should not be needed
            if let Some(sub_type_name) = sub_type.names.first() {
                elements.push(toks![
                    "#[serde(rename = ",
                    sub_type_name.to_string().quoted(),
                    ")]",
                ]);
            }

            elements.push(toks![sub_type.local_name.as_str(), " {"]);

            for field in body.fields.iter().chain(sub_type.fields.iter()) {
                elements.nested(self.field_element(field)?);
            }

            elements.push("},");

            t.push(elements);

            Ok(()) as Result<()>
        })?;

        t.push("}");

        out.0.push(t);

        Ok(())
    }
}

impl PackageUtils for RustBackend {}
