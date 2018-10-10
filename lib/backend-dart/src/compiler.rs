//! Backend for Dart

use backend::PackageProcessor;
use core::errors::*;
use core::{self, Handle, Loc, RelativePath, RelativePathBuf};
use flavored::{
    RpEnumBody, RpField, RpInterfaceBody, RpName, RpPackage, RpServiceBody, RpTupleBody,
    RpTypeBody, RpVariant, DartFlavor,
};
use genco::dart;
use genco::{Cons, IntoTokens, Quoted, Dart, Tokens};
use dart_file_spec::DartFileSpec;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;
use trans::{self, Translated};
use utils::Comments;
use {Options, Service, EXT, LIB, MOD, TYPE_SEP};

/// #[allow(non_camel_case_types)] attribute.
pub struct AllowNonCamelCaseTypes;

impl<'a> IntoTokens<'a, Dart<'a>> for AllowNonCamelCaseTypes {
    fn into_tokens(self) -> Tokens<'a, Dart<'a>> {
        "#[allow(non_camel_case_types)]".into()
    }
}

/// Serializer derives.
pub struct Derives;

impl<'a> IntoTokens<'a, Dart<'a>> for Derives {
    fn into_tokens(self) -> Tokens<'a, Dart<'a>> {
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]".into()
    }
}

pub struct EnumDerives;

impl<'a> IntoTokens<'a, Dart<'a>> for EnumDerives {
    fn into_tokens(self) -> Tokens<'a, Dart<'a>> {
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]".into()
    }
}

/// A serde rename annotation.
pub struct Rename<'a>(&'a str);

impl<'a> IntoTokens<'a, Dart<'a>> for Rename<'a> {
    fn into_tokens(self) -> Tokens<'a, Dart<'a>> {
        toks!["#[serde(rename = ", self.0.quoted(), ")]"]
    }
}

/// Tag attribute.
pub struct Tag<'a>(&'a str);

impl<'a> IntoTokens<'a, Dart<'a>> for Tag<'a> {
    fn into_tokens(self) -> Tokens<'a, Dart<'a>> {
        toks!["#[serde(tag = ", self.0.quoted(), ")]"]
    }
}

/// Untagged attribute.
pub struct Untagged;

impl<'el> IntoTokens<'el, Dart<'el>> for Untagged {
    fn into_tokens(self) -> Tokens<'el, Dart<'el>> {
        toks!["#[serde(untagged)]"]
    }
}

pub struct Compiler<'el> {
    pub env: &'el Translated<DartFlavor>,
    options: Options,
    handle: &'el Handle,
}

impl<'el> Compiler<'el> {
    pub fn new(
        env: &'el Translated<DartFlavor>,
        options: Options,
        handle: &'el Handle,
    ) -> Compiler<'el> {
        Compiler {
            env,
            options,
            handle,
        }
    }

    /// Build an implementation of the given name and body.
    fn build_impl(&self, name: Rc<String>, body: Tokens<'el, Dart<'el>>) -> Tokens<'el, Dart<'el>> {
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
    fn convert_type_name(&self, name: &RpName) -> (Rc<String>, Tokens<'static, Dart<'static>>) {
        let attributes = if name.path.len() > 1 {
            AllowNonCamelCaseTypes.into_tokens()
        } else {
            Tokens::new()
        };

        (Rc::new(name.join(TYPE_SEP)), attributes)
    }

    fn into_type<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Dart<'a>>> {
        let stmt = toks![field.ty.clone()];

        if field.is_optional() {
            return Ok(toks!["Option<", stmt, ">"]);
        }

        Ok(stmt)
    }

    fn enum_value_fn<'a>(
        &self,
        body: &'a RpEnumBody,
        name: Rc<String>,
        match_body: Tokens<'a, Dart<'a>>,
    ) -> Tokens<'a, Dart<'a>> {
        let mut value_fn = Tokens::new();
        let mut match_decl = Tokens::new();

        match_decl.push("match *self {");
        match_decl.nested(match_body);
        match_decl.push("}");

        push!(value_fn, "pub fn value(&self) -> ", body.enum_type, " {");
        value_fn.nested(toks!["use self::", name, "::*;"]);
        value_fn.nested(match_decl);
        value_fn.push("}");

        value_fn
    }

    // Build the corresponding element out of a field declaration.
    fn field_element<'a>(&self, field: &'a RpField, is_pub: bool) -> Result<Tokens<'a, Dart<'a>>> {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let type_spec = self.into_type(field)?;

        if field.is_optional() {
            t.push(toks!["#[serde(skip_serializing_if=\"Option::is_none\")]"]);
        }

        if field.name() != ident {
            t.push(Rename(field.name()));
        }

        t.push_into(|t| {
            if is_pub {
                t.append("pub ");
            }

            t.append(toks![ident, ": ", type_spec, ","]);
        });

        Ok(t.into())
    }

    pub fn compile(&self) -> Result<()> {
        let mut files = self.populate_files()?;
        self.write_mod_files(&files)?;
        self.write_files(files)
    }

    fn write_mod_files(&self, files: &BTreeMap<RpPackage, DartFileSpec>) -> Result<()> {
        let mut packages: BTreeMap<RelativePathBuf, BTreeSet<String>> = BTreeMap::new();
        let mut root_names = BTreeSet::new();

        for (package, _) in files {
            let mut current = RelativePathBuf::new();

            let mut it = package.parts().peekable();

            if let Some(root) = it.peek() {
                root_names.insert(root.to_string());
            }

            while let Some(part) = it.next() {
                current = current.join(part);

                if let Some(next) = it.peek() {
                    let mut full_path = current.join(MOD);
                    full_path.set_extension(self.ext());

                    packages
                        .entry(full_path)
                        .or_insert_with(BTreeSet::new)
                        .insert(next.to_string());
                }
            }
        }

        let mut root_mod = RelativePathBuf::new().join(MOD);
        root_mod.set_extension(self.ext());
        packages.insert(root_mod, root_names);

        let handle = self.handle();

        for (full_path, children) in packages {
            // skip writing mod if lib file exists.
            let lib_path = full_path.with_file_name(LIB).with_extension(self.ext());
            let parent = full_path.parent().unwrap_or(RelativePath::new("."));

            if !self.handle.is_dir(&parent) {
                debug!("+dir: {}", parent.display());
                handle.create_dir_all(&parent)?;
            }

            // do not create mod file if there is a lib.rs in the root.
            if lib_path.parent().is_none() && handle.is_file(&lib_path) {
                debug!(
                    "+mod: {} (skip due to: {})",
                    full_path.display(),
                    lib_path.display()
                );
                continue;
            }

            if !handle.is_file(&full_path) {
                debug!("+mod: {}", full_path.display());
                let mut f = handle.create(&full_path)?;

                for child in children {
                    writeln!(f, "pub mod {};", child)?;
                }
            }
        }

        Ok(())
    }
}

impl<'el> PackageProcessor<'el, DartFlavor, Loc<RpName>> for Compiler<'el> {
    type Out = DartFileSpec<'el>;
    type DeclIter = trans::translated::DeclIter<'el, DartFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &'el Handle {
        self.handle
    }

    fn default_process(&self, _out: &mut Self::Out, _: &Loc<RpName>) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody) -> Result<()> {
        let mut fields = Tokens::new();

        for field in &body.fields {
            fields.append(toks!["pub ", self.into_type(field)?]);
        }

        let (name, attributes) = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push_unless_empty(attributes);
        t.push(Derives);
        t.push(toks!["pub struct ", name, "(", fields.join(", "), ");",]);

        out.0.push(t);
        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        let (name, mut attributes) = self.convert_type_name(&body.name);

        // variant declarations
        let mut vars = Tokens::new();
        // body of value function
        let mut match_body = Tokens::new();

        if let core::RpVariants::Number { .. } = body.variants {
            attributes.push(EnumDerives);
        } else {
            attributes.push(Derives);
        }

        for v in body.variants.iter() {
            vars.push_unless_empty(Comments(&v.comment));

            match v.value {
                core::RpVariantValue::String(string) => {
                    if string != v.ident() {
                        vars.push(Rename(string));
                    }

                    push!(vars, v.ident(), ",");
                    push!(match_body, v.ident(), " => ", string.quoted(), ",");
                }
                core::RpVariantValue::Number(number) => {
                    push!(vars, v.ident(), ",");
                    push!(match_body, v.ident(), " => ", number.to_string(), ",");
                }
            }
        }

        out.0.push({
            let mut t = Tokens::new();

            t.push_unless_empty(Comments(&body.comment));
            t.push_unless_empty(attributes);
            t.push(toks!["pub enum ", name.clone(), " {"]);
            t.nested(vars);
            t.push("}");

            t
        });

        out.0.push({
            let mut t = Tokens::new();

            t.push(toks!["impl ", name.clone(), " {"]);

            t.nested({
                let mut t = Tokens::new();
                t.push(self.enum_value_fn(body, name.clone(), match_body));
                t.push_unless_empty(code!(&body.codes, core::RpContext::Dart));
                t
            });

            t.push("}");

            t
        });

        // Serialize impl for numerics.
        if let core::RpVariants::Number { ref variants } = body.variants {
        }

        return Ok(());
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
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
                t.push({
                    let mut t = Tokens::new();
                    t.push_unless_empty(Comments(&field.comment));
                    t.push(self.field_element(field, true)?);
                    t
                });
            }

            t.join_line_spacing()
        });

        t.push("}");

        out.0.push(t);

        // if custom code is present, punt it into an impl.
        let impl_body = code!(&body.codes, core::RpContext::Dart).into_tokens();

        if !impl_body.is_empty() {
            out.0.push(self.build_impl(name.clone(), impl_body));
        }

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push_unless_empty(attributes);
        t.push(Derives);

        match body.sub_type_strategy {
            core::RpSubTypeStrategy::Tagged { ref tag, .. } => {
                t.push(Tag(tag.as_str()));
            }
            core::RpSubTypeStrategy::Untagged => {
                t.push(Untagged);
            }
        }

        t.push(toks!["pub enum ", name.clone(), " {"]);

        for s in &body.sub_types {
            t.nested({
                let mut t = Tokens::new();

                t.push_unless_empty(Comments(&s.comment));

                // TODO: clone should not be needed
                if let Some(ref name) = s.sub_type_name {
                    if name.as_str() != s.ident.as_str() {
                        t.push(Rename(name));
                    }
                }

                t.push(toks![s.ident.as_str(), " {"]);

                t.push({
                    let mut t = Tokens::new();

                    for field in body.fields.iter().chain(s.fields.iter()) {
                        t.nested({
                            let mut t = Tokens::new();
                            t.push_unless_empty(Comments(&field.comment));
                            t.push(self.field_element(field, false)?);
                            t
                        });
                    }

                    t.join_line_spacing()
                });

                t.push("},");

                t
            });
        }

        t.push("}");

        out.0.push(t);

        let impl_body = code!(&body.codes, core::RpContext::Dart).into_tokens();

        if !impl_body.is_empty() {
            out.0.push(self.build_impl(name.clone(), impl_body));
        }

        Ok(())
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el RpServiceBody) -> Result<()> {
        Ok(())
    }
}
