//! Backend for Dart

use backend::PackageProcessor;
use core::errors::*;
use core::{self, Handle, Loc};
use dart_file_spec::DartFileSpec;
use flavored::{
    DartFlavor, RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody,
    RpTupleBody, RpTypeBody,
};
use genco::{dart, Dart, IntoTokens, Quoted, Tokens};
use std::rc::Rc;
use trans::{self, Translated};
use utils::Comments;
use {EXT, TYPE_SEP};

pub struct Compiler<'el> {
    pub env: &'el Translated<DartFlavor>,
    handle: &'el Handle,
    map_of_strings: Dart<'el>,
    list_of_dynamic: Dart<'el>,
}

impl<'el> Compiler<'el> {
    pub fn new(env: &'el Translated<DartFlavor>, handle: &'el Handle) -> Compiler<'el> {
        let string = dart::imported("dart:core").name("String");
        let map = dart::imported("dart:core").name("Map");
        let list = dart::imported("dart:core").name("list");
        let map_of_strings = map.with_arguments(vec![string, Dart::Dynamic]);
        let list_of_dynamic = list.with_arguments(vec![Dart::Dynamic]);
        Compiler { env, handle, map_of_strings, list_of_dynamic }
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
    fn convert_type_name(&self, name: &RpName) -> Rc<String> {
        Rc::new(name.join(TYPE_SEP))
    }

    /// Convert field into type.
    fn into_type<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Dart<'a>>> {
        Ok(toks!(field.ty.clone()))
    }

    // Build the corresponding element out of a field declaration.
    fn field_element<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Dart<'a>>> {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let type_spec = self.into_type(field)?;

        t.push_into(|t| {
            t.append(toks![type_spec, " ", ident, ";"]);
        });

        Ok(t.into())
    }

    pub fn compile(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_files(files)
    }

    /// Build field declarations for the given fields.
    pub fn type_fields(&self, fields: impl IntoIterator<Item = &'el Loc<RpField>>) -> Result<Tokens<'el, Dart<'el>>> {
        let mut t = Tokens::new();

        for field in fields {
            t.push({
                let mut t = Tokens::new();
                t.push_unless_empty(Comments(&field.comment));
                t.push(self.field_element(field)?);
                t
            });
        }

        Ok(t)
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
        let name = self.convert_type_name(&body.name);

        let decode_fn = {
            let mut t = Tokens::new();
            push!(t, "static ", name, " decode(dynamic data) {");

            t.nested({
                let mut t = Tokens::new();

                t.push({
                    let mut t = Tokens::new();
                    push!(t, "if (!(data is ", self.list_of_dynamic, ")) {");
                    nested!(t, "throw 'expected ", self.list_of_dynamic, ", but got: $data';");
                    push!(t, "}");
                    t
                });

                t.push({
                    let mut t = Tokens::new();

                    for _field in &body.fields {
                        // TODO: decode each field.
                    }

                    t
                });

                t.join_line_spacing()
            });

            push!(t, "}");
            t
        };

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push(toks!("class ", name, "{"));
        t.nested({
            let mut t = Tokens::new();
            t.push(self.type_fields(&body.fields)?);
            t.push(decode_fn);
            t.join_line_spacing()
        });
        t.push(toks!("}"));

        out.0.push(t);
        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);

        // variant declarations
        let mut fields = Tokens::new();

        for v in body.variants.iter() {
            fields.push_unless_empty(Comments(&v.comment));
            let field = toks!("static const ", v.ident());

            match v.value {
                core::RpVariantValue::String(string) => {
                    push!(fields, field, " = const ", name, "._new(", string.quoted(), ");");
                }
                core::RpVariantValue::Number(number) => {
                    push!(fields, field, " = const ", name, "._new(", number.to_string(), ");");
                }
            }
        }

        let f = "_value";

        let decode_fn = {
            let mut t = Tokens::new();
            push!(t, "static ", name, " decode(dynamic data) {");

            t.nested({
                let mut t = Tokens::new();

                t.push({
                    let mut t = Tokens::new();
                    push!(t, "if (!(data is ", body.enum_type, ")) {");
                    nested!(t, "throw 'expected ", body.enum_type, ", but got: $data';");
                    push!(t, "}");
                    t
                });

                t.push({
                    let mut t = Tokens::new();

                    push!(t, "switch (data as ", body.enum_type, ") {");

                    for v in body.variants.iter() {
                        let m = match v.value {
                            core::RpVariantValue::String(string) => toks!(string.quoted()),
                            core::RpVariantValue::Number(number) => toks!(number.to_string()),
                        };
                        push!(t, "case ", m, ":");
                        nested!(t, "return ", name, ".", v.ident(), ";");
                    }

                    push!(t, "default:");
                    nested!(t, "throw 'unexpected ", name, " value: $data';");

                    push!(t, "}");

                    t
                });

                t.join_line_spacing()
            });

            push!(t, "}");
            t
        };

        let encode_fn = {
            let mut t = Tokens::new();
            push!(t, "dynamic encode() {");
            nested!(t, "return ", f, ";");
            push!(t, "}");
            t
        };

        out.0.push({
            let mut t = Tokens::new();

            t.push_unless_empty(Comments(&body.comment));
            t.push(toks!["class ", name.clone(), " {"]);
            t.nested({
                let mut t = Tokens::new();
                t.push({
                    let mut t = Tokens::new();
                    push!(t, "final ", f, ";");
                    push!(t, "const ", name, "._new(this.", f, ");");
                    push!(t, "toString() => '", name, ".$", f, "';");
                    t
                });
                t.push(fields);
                t.push(decode_fn);
                t.push(encode_fn);
                t.join_line_spacing()
            });
            t.push("}");

            t
        });

        return Ok(());
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);

        let decode_fn = {
            let mut t = Tokens::new();
            push!(t, "static ", name, " decode(dynamic data) {");

            t.nested({
                let mut t = Tokens::new();

                t.push({
                    let mut t = Tokens::new();
                    push!(t, "if (!(data is ", self.map_of_strings, ")) {");
                    nested!(t, "throw 'expected ", self.map_of_strings, ", but got: $data';");
                    push!(t, "}");
                    t
                });

                t.push({
                    let mut t = Tokens::new();

                    for _field in &body.fields {
                        // TODO: decode each field.
                    }

                    t
                });

                t.join_line_spacing()
            });

            push!(t, "}");
            t
        };

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push(toks!["class ", name.clone(), " {"]);

        // fields
        t.nested({
            let mut t = Tokens::new();
            t.push(self.type_fields(&body.fields)?);
            t.push(decode_fn);
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
        let name = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));

        match body.sub_type_strategy {
            core::RpSubTypeStrategy::Tagged { .. } => {
            }
            core::RpSubTypeStrategy::Untagged => {
            }
        }

        t.push(toks!["pub enum ", name.clone(), " {"]);

        for s in &body.sub_types {
            t.nested({
                let mut t = Tokens::new();

                t.push_unless_empty(Comments(&s.comment));

                t.push(toks![s.ident.as_str(), " {"]);

                t.push({
                    let mut t = Tokens::new();
                    t.nested(self.type_fields(body.fields.iter().chain(s.fields.iter()))?);
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

    fn process_service(&self, _: &mut Self::Out, _: &'el RpServiceBody) -> Result<()> {
        Ok(())
    }
}
