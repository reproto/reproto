//! Backend for Rust

use crate::flavored::{
    Primitive, RpEnumBody, RpField, RpInterfaceBody, RpName, RpPackage, RpServiceBody, RpTupleBody,
    RpTypeBody, RpVariant, RpVariants, RustFlavor, Type,
};
use crate::utils::Comments;
use crate::{Options, Root, Service, EXT, MOD, TYPE_SEP};
use backend::PackageProcessor;
use core::errors::*;
use core::{self, Handle, Loc, RelativePathBuf};
use genco::prelude::*;
use genco::tokens::FormatInto;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;
use trans::{self, Translated};

/// Serializer derives.
pub struct Derives {
    serialize: rust::Import,
    deserialize: rust::Import,
}

impl Derives {
    pub fn new() -> Self {
        Self {
            serialize: rust::import("serde", "Serialize").direct(),
            deserialize: rust::import("serde", "Deserialize").direct(),
        }
    }
}

impl<'a> FormatInto<Rust> for &'a Derives {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        quote_in! { *tokens => #[derive(Clone, Debug, PartialEq, #(&self.serialize), #(&self.deserialize))] }
    }
}

pub struct EnumDerives;

impl<'el> FormatInto<Rust> for EnumDerives {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        quote_in! { *tokens => #[derive(Clone, Copy, Debug, PartialEq, Eq)] }
    }
}

/// A serde rename annotation.
pub struct Rename<'a>(&'a str);

impl<'el> FormatInto<Rust> for Rename<'el> {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        quote_in!(*tokens => #[serde(rename = #(quoted(self.0)))])
    }
}

/// Tag attribute.
pub struct Tag<'a>(&'a str);

impl<'el> FormatInto<Rust> for Tag<'el> {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        quote_in!(*tokens => #[serde(tag = #(quoted(self.0)))])
    }
}

/// Untagged attribute.
pub struct Untagged;

impl<'el> FormatInto<Rust> for Untagged {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        quote_in!(*tokens => #[serde(untagged)])
    }
}

pub struct Compiler<'el> {
    pub env: &'el Translated<RustFlavor>,
    options: Options,
    handle: &'el dyn Handle,
    derives: Derives,
}

impl<'el> Compiler<'el> {
    pub fn new(
        env: &'el Translated<RustFlavor>,
        options: Options,
        handle: &'el dyn Handle,
    ) -> Compiler<'el> {
        Compiler {
            env,
            options,
            handle,
            derives: Derives::new(),
        }
    }

    /// Convert the type name
    ///
    /// Optionally also emit the necessary attributes to suppress warnings for bad naming
    /// conventions.
    fn convert_type_name(&self, name: &RpName) -> (Rc<String>, Tokens<Rust>) {
        let attributes = if name.path.len() > 1 {
            quote!(#[allow(non_camel_case_types)])
        } else {
            Tokens::new()
        };

        (Rc::new(name.join(TYPE_SEP)), attributes)
    }

    fn write_type(&self, out: &mut Tokens<Rust>, field: &RpField) {
        if field.is_optional() {
            quote_in!(*out => Option<#(&field.ty)>)
        } else {
            quote_in!(*out => #(&field.ty))
        }
    }

    fn enum_value_fn<'a>(
        &self,
        out: &mut Tokens<Rust>,
        body: &'a RpEnumBody,
        variants: &RpVariants,
    ) {
        quote_in! { *out =>
            pub fn value(&self) -> #(&body.enum_type) {
                match self {
                    #(for v in variants join (#<push>) =>
                        #(match v.value {
                            core::RpVariantValue::String(string) => {
                                Self::#(v.ident()) => #(quoted(string)),
                            }
                            core::RpVariantValue::Number(number) => {
                                Self::#(v.ident()) => #(display(number)),
                            }
                        })
                    )
                }
            }
        }
    }

    // Build the corresponding element out of a field declaration.
    fn field_element(&self, out: &mut Tokens<Rust>, field: &RpField) {
        let ident = field.safe_ident().to_string();

        quote_in! { *out =>
            #(if field.is_optional() {
                #[serde(skip_serializing_if="Option::is_none")]
            })
            #(if field.name() != ident {
                #(Rename(field.name()))
            })
            pub #ident: #(ref out => self.write_type(out, field))
        }
    }

    pub fn compile(&self) -> Result<()> {
        use genco::fmt;

        let mut files = self.do_populate_files(|_, new, out| {
            // Add a line separating entries in each non-new file.
            if !new {
                out.line();
            }

            Ok(())
        })?;

        for g in &self.options.root {
            g.generate(Root { files: &mut files })?;
        }

        self.write_mod_files(&files)?;

        let handle = self.handle();

        for (package, out) in files {
            let full_path = self.setup_module_path(&package)?;

            log::debug!("+module: {}", full_path);

            let mut w = fmt::IoWriter::new(handle.create(&full_path)?);
            let config = rust::Config::default().with_default_import(rust::ImportMode::Qualified);
            let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(2));

            out.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }

    fn write_mod_files(&self, files: &BTreeMap<RpPackage, rust::Tokens>) -> Result<()> {
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

        let root_mod = RelativePathBuf::new().join(MOD).with_extension(self.ext());
        packages.insert(root_mod, root_names);

        let handle = self.handle();

        for (full_path, children) in packages {
            if let Some(parent) = full_path.parent() {
                if !self.handle.is_dir(parent) {
                    log::debug!("+dir: {}", parent);
                    handle.create_dir_all(&parent)?;
                }
            }

            log::debug!("+mod: {}", full_path);
            let mut f = handle.create(&full_path)?;

            for child in children {
                writeln!(f, "pub mod {};", child)?;
            }
        }

        Ok(())
    }
}

impl<'el> PackageProcessor<'el, RustFlavor, Loc<RpName>> for Compiler<'el> {
    type Out = rust::Tokens;
    type DeclIter = trans::translated::DeclIter<'el, RustFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &'el dyn Handle {
        self.handle
    }

    fn default_process(&self, _out: &mut Self::Out, _: &Loc<RpName>) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);

        quote_in! { *out =>
            #<line>
            #(Comments(&body.comment))
            #attributes
            #(&self.derives)
            pub struct #name(#(for f in &body.fields join (, ) => pub #(ref out => self.write_type(out, f))));
        };

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        let (name, mut attributes) = self.convert_type_name(&body.name);
        let name = &name;

        if let core::RpVariants::Number { .. } = body.variants {
            // TODO: commented out, see: https://github.com/rust-lang/rust/issues/49973
            // enable through option?
            // attributes.push(Repr(body.enum_type.clone()));
            attributes.push();
            attributes.append(EnumDerives);
        } else {
            attributes.push();
            attributes.append(&self.derives);
        }

        quote_in! { *out =>
            #(Comments(&body.comment))
            #attributes
            pub enum #name {
                #(for v in &body.variants join (#<push>) =>
                    #(Comments(v.comment))
                    #(match v.value {
                        core::RpVariantValue::String(string) if string != v.ident() => {
                            #(Rename(string))
                        }
                        _ => {}
                    })
                    #(v.ident()),
                )
            }

            impl #name {
                #(ref out => self.enum_value_fn(out, body, &body.variants))

                #(if backend::code_contains!(body.codes, core::RpContext::Rust) {
                    #(ref out => backend::code_in!(out, &body.codes, core::RpContext::Rust))
                })
            }

            #(if let core::RpVariants::Number { variants } = &body.variants {
                #(ref t => numeric_serialize(t, body, &name, variants))

                #(ref t => numeric_deserialize(t, body, &name, variants))
            })
        }

        return Ok(());

        /// Build a numeric serialize implementation.
        fn numeric_serialize<'el, T>(
            out: &mut Tokens<Rust>,
            body: &'el RpEnumBody,
            name: &Rc<String>,
            variants: &'el Vec<Loc<RpVariant<T>>>,
        ) where
            T: fmt::Display,
        {
            let ser = rust::import("serde", "Serialize");
            let serializer = rust::import("serde", "Serializer");
            let ty = &body.enum_type;

            quote_in! { *out =>
                impl #ser for #name {
                    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                    where
                        S: #serializer
                    {
                        let o = match self {
                            #(for v in variants =>
                                #name::#(v.ident()) => #(v.value.to_string())#(ty),#<push>
                            )
                        };

                        s.serialize_#(&body.enum_type)(o)
                    }
                }
            }
        }

        /// Build a numeric deserialize implementation.
        fn numeric_deserialize<'el, T>(
            out: &mut Tokens<Rust>,
            body: &'el RpEnumBody,
            name: &Rc<String>,
            variants: &'el Vec<Loc<RpVariant<T>>>,
        ) where
            T: fmt::Display,
        {
            let des = rust::import("serde", "Deserialize");
            let deserializer = rust::import("serde", "Deserializer");

            quote_in! { *out =>
                impl<'de> #des<'de> for #name {
                    fn deserialize<D>(d: D) -> Result<#name, D::Error>
                    where
                        D: #deserializer<'de>
                    {
                        #(ref out => numeric_visitor_deserialize(out, body, name, variants, "Visitor"))

                        d.deserialize_#(&body.enum_type)(Visitor)
                    }
                }
            }
        }

        /// Build a numeric deserialize visitor.
        fn numeric_visitor_deserialize<'el, T>(
            out: &mut Tokens<Rust>,
            body: &'el RpEnumBody,
            parent: &Rc<String>,
            variants: &'el Vec<Loc<RpVariant<T>>>,
            name: &'el str,
        ) where
            T: fmt::Display,
        {
            let visitor = rust::import("serde::de", "Visitor");

            quote_in! { *out =>
                struct Visitor;

                impl<'de> #visitor<'de> for #name {
                    type Value = #parent;

                    #(ref out => {
                        numeric_expecting(out, parent, variants);
                        out.line();
                        numeric_visit(out, &body.enum_type, parent, variants);

                        if body.enum_type == Type::Primitive(Primitive::I32) {
                            out.line();
                            forward(out, &Type::Primitive(Primitive::I64), &body.enum_type, parent);
                            out.line();
                            forward(out, &Type::Primitive(Primitive::U64), &body.enum_type, parent);
                        }

                        if body.enum_type == Type::Primitive(Primitive::I64) {
                            out.line();
                            forward(out, &Type::Primitive(Primitive::U64), &body.enum_type, parent);
                        }

                        if body.enum_type == Type::Primitive(Primitive::U32) {
                            out.line();
                            forward(out, &Type::Primitive(Primitive::U64), &body.enum_type, parent);
                        }
                    })
                }
            }
        }

        fn numeric_expecting<'el, T>(
            out: &mut Tokens<Rust>,
            parent: &Rc<String>,
            variants: &'el Vec<Loc<RpVariant<T>>>,
        ) where
            T: fmt::Display,
        {
            let variants = variants
                .iter()
                .map(|v| v.value.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            let res = rust::import("std::fmt", "Result");
            let fmt = rust::import("std::fmt", "Formatter");
            let m = quoted(format!("{}, one of: {}", parent, variants));

            quote_in! { *out =>
                fn expecting(&self, fmt: &mut #fmt) -> #res {
                    fmt.write_str(#m)
                }
            }
        }

        fn numeric_visit<'el, T>(
            out: &mut Tokens<Rust>,
            ty: &'el Type,
            parent: &Rc<String>,
            variants: &'el Vec<Loc<RpVariant<T>>>,
        ) where
            T: fmt::Display,
        {
            let err = rust::import("serde::de", "Error");
            let error_fmt = quoted(format!("{}: unknown value: {{}}", parent.as_str()));

            quote_in! { *out =>
                fn visit_#(ty)<E>(self, value: #ty) -> Result<#parent, E>
                    where E: #err
                {
                    match value {
                        #(for v in &*variants join(,#<push>) =>
                            #(v.value.to_string())#ty => Ok(#(parent.clone())::#(v.ident()))
                        ),
                        value => Err(E::custom(format!(#error_fmt, value))),
                    }
                }
            }
        }

        fn forward<'el>(
            out: &mut rust::Tokens,
            ty: &'el Type,
            forward_ty: &'el Type,
            parent: &Rc<String>,
        ) {
            out.line();

            quote_in! { *out =>
                fn visit_#(ty)<E>(self, value: #ty) -> Result<#(parent.clone()), E>
                    where E: #(rust::import("serde::de", "Error"))
                {
                    self.visit_#(forward_ty)(value as #forward_ty)
                }
            }
        }
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);
        let name = &name;

        quote_in! { *out =>
            #<line>
            #(Comments(&body.comment))
            #attributes
            #(&self.derives)
            pub struct #name {
                #(for field in &body.fields join (#<line>) =>
                    #(Comments(&field.comment))
                    #(ref out => self.field_element(out, field)),
                )
            }

            #(if backend::code_contains!(body.codes, core::RpContext::Rust) {
                impl #name {
                    #(ref out => backend::code_in!(out, &body.codes, core::RpContext::Rust))
                }
            })
        };

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);

        quote_in! { *out =>
            #(Comments(&body.comment))
            #(attributes)
            #(&self.derives)
            #(match &body.sub_type_strategy {
                core::RpSubTypeStrategy::Tagged { tag, .. } => #(Tag(tag.as_str())),
                core::RpSubTypeStrategy::Untagged => #Untagged,
            })
            pub enum #(&name) {
                #(for s in &body.sub_types join (#<line>) => #(ref out =>
                    let (sub_name, _) = self.convert_type_name(&s.name);

                    if let Some(name) = &s.sub_type_name {
                        if name.as_str() != s.ident.as_str() {
                            out.push();
                            out.append(Rename(name));
                        }
                    }

                    out.push();
                    quote_in!(*out => #(&s.ident)(#sub_name),);
                ))
            }

            #(if backend::code_contains!(body.codes, core::RpContext::Rust) {
                impl #name {
                    #(ref out => backend::code_in!(out, &body.codes, core::RpContext::Rust))
                }
            })

            #(for s in &body.sub_types join (#<line>) => #(ref out =>
                let (sub_name, attributes) = self.convert_type_name(&s.name);

                quote_in! { *out =>
                    #(Comments(&s.comment))
                    #(&self.derives)
                    #attributes
                    pub struct #sub_name {
                        #(for field in body.fields.iter().chain(&s.fields) join (#<line>) =>
                            #(Comments(&field.comment))
                            #(ref out => self.field_element(out, field)),
                        )
                    }
                }
            ))
        };

        Ok(())
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el RpServiceBody) -> Result<()> {
        let (name, attributes) = self.convert_type_name(&body.name);

        for s in &self.options.service {
            s.generate(Service {
                body,
                container: out,
                name: name.clone().into(),
                attributes: &attributes,
            })?;
        }

        Ok(())
    }
}
