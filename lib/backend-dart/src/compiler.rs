//! Backend for Dart

use backend::PackageProcessor;
use core::errors::*;
use core::{self, Handle, Loc};
use dart_file_spec::DartFileSpec;
use flavored::{
    DartFlavor, RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody,
    RpTupleBody, RpTypeBody,
};
use genco::{Dart, IntoTokens, Quoted, Tokens};
use std::rc::Rc;
use trans::{self, Translated};
use utils::Comments;
use {EXT, TYPE_SEP};

pub struct Compiler<'el> {
    pub env: &'el Translated<DartFlavor>,
    handle: &'el Handle,
}

impl<'el> Compiler<'el> {
    pub fn new(env: &'el Translated<DartFlavor>, handle: &'el Handle) -> Compiler<'el> {
        Compiler { env, handle }
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

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push(toks!["class ", name, "{", "}"]);

        out.0.push(t);
        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);

        // variant declarations
        let mut vars = Tokens::new();
        // body of value function
        let mut match_body = Tokens::new();

        for v in body.variants.iter() {
            vars.push_unless_empty(Comments(&v.comment));

            match v.value {
                core::RpVariantValue::String(string) => {
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

        return Ok(());
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);
        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push(toks!["class ", name.clone(), " {"]);

        // fields
        t.nested({
            let mut t = Tokens::new();

            for field in &body.fields {
                t.push({
                    let mut t = Tokens::new();
                    t.push_unless_empty(Comments(&field.comment));
                    t.push(self.field_element(field)?);
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

                    for field in body.fields.iter().chain(s.fields.iter()) {
                        t.nested({
                            let mut t = Tokens::new();
                            t.push_unless_empty(Comments(&field.comment));
                            t.push(self.field_element(field)?);
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

    fn process_service(&self, _: &mut Self::Out, _: &'el RpServiceBody) -> Result<()> {
        Ok(())
    }
}
