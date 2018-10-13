//! Backend for Dart

use backend::PackageProcessor;
use core::errors::*;
use core::{self, Handle, Loc};
use dart_file_spec::DartFileSpec;
use flavored::{
    DartFlavor, RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody, RpTupleBody,
    RpTypeBody,
};
use genco::{dart, Cons, Dart, Element, Quoted, Tokens};
use std::rc::Rc;
use trans::{self, Translated};
use utils::{AssertNotNull, AssertType, Comments};
use {EXT, TYPE_SEP};

pub struct Compiler<'el> {
    pub env: &'el Translated<DartFlavor>,
    handle: &'el Handle,
    map_of_strings: Dart<'el>,
    list_of_dynamic: Dart<'el>,
}

impl<'el> Compiler<'el> {
    pub fn new(env: &'el Translated<DartFlavor>, handle: &'el Handle) -> Compiler<'el> {
        let core = dart::imported(dart::DART_CORE);
        let string = core.name("String");
        let map = core.name("Map");
        let list = core.name("List");
        let map_of_strings = map.with_arguments(vec![string.clone(), Dart::Dynamic]);
        let list_of_dynamic = list.with_arguments(vec![Dart::Dynamic]);

        Compiler {
            env,
            handle,
            map_of_strings,
            list_of_dynamic,
        }
    }

    /// Convert the type name
    fn convert_type_name(&self, name: &RpName) -> Cons<'static> {
        Cons::from(name.join(TYPE_SEP))
    }

    // Build the corresponding element out of a field declaration.
    fn field_element<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Dart<'a>>> {
        Ok(toks![field.ty.ty(), " ", field.safe_ident(), ";"])
    }

    pub fn compile(&self) -> Result<()> {
        let files = self.populate_files()?;
        self.write_files(files)
    }

    /// Build field declarations for the given fields.
    fn type_fields(
        &self,
        fields: impl IntoIterator<Item = &'el Loc<RpField>>,
    ) -> Result<Tokens<'el, Dart<'el>>> {
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

    /// Build a decode function.
    fn decode_fn(
        &self,
        name: Cons<'el>,
        fields: impl IntoIterator<Item = &'el Loc<RpField>>,
    ) -> Result<Tokens<'el, Dart<'el>>> {
        let mut t = Tokens::new();
        push!(t, "static ", name, " decode(dynamic _dataDyn) {");

        t.nested({
            let mut t = Tokens::new();

            t.push(AssertType(self.map_of_strings.clone(), "_dataDyn"));
            push!(t, self.map_of_strings, " _data = _dataDyn;");

            t.push({
                let mut t = Tokens::new();
                let mut vars = toks!();

                for field in fields {
                    let id = field.safe_ident();
                    let id_dyn = Cons::from(Rc::new(format!("{}_dyn", field.safe_ident())));

                    t.push({
                        let mut t = Tokens::new();

                        push!(t, "var ", id_dyn, " = _data[", field.name().quoted(), "];");

                        if field.is_optional() {
                            t.push({
                                let mut t = Tokens::new();
                                push!(t, field.ty.ty(), " ", id, " = null;");
                                push!(t, "if (", id_dyn, " != null) {");

                                t.nested({
                                    let mut t = toks!();

                                    let (d, e) = field.ty.decode(id_dyn)?;
                                    t.push_unless_empty(e);
                                    push!(t, id, " = ", d, ";");
                                    t
                                });

                                t.push("}");
                                t
                            });
                        } else {
                            t.push({
                                let (d, e) = field.ty.decode(id_dyn.clone())?;
                                let mut t = toks!();
                                t.push(AssertNotNull(id_dyn));
                                t.push_unless_empty(e);
                                push!(t, "final ", field.ty.ty(), " ", id, " = ", d, ";");
                                t
                            });
                        }

                        t
                    });

                    vars.append(id);
                }

                push!(t, "return ", name, "(", vars.join(", "), ");");
                t.join_line_spacing()
            });

            t.join_line_spacing()
        });

        push!(t, "}");
        Ok(t)
    }

    /// Build a tuple decode function.
    fn decode_tuple_fn(
        &self,
        name: Cons<'el>,
        fields: impl Clone + IntoIterator<Item = &'el Loc<RpField>>,
    ) -> Result<Tokens<'el, Dart<'el>>> {
        let mut t = Tokens::new();
        push!(t, "static ", name, " decode(dynamic _dataDyn) {");

        t.nested({
            let mut t = Tokens::new();

            t.push(AssertType(self.list_of_dynamic.clone(), "_dataDyn"));
            push!(t, self.list_of_dynamic, " _data = _dataDyn;");

            t.push({
                let mut t = Tokens::new();
                let mut vars = toks!();
                let len = Cons::from(fields.clone().into_iter().count().to_string());

                t.push({
                    let mut t = toks!();
                    push!(t, "if (_data.length != ", len, ") {");
                    nested!(
                        t,
                        "throw 'expected array of length ",
                        len,
                        ", but was $_data.length';"
                    );
                    push!(t, "}");
                    t
                });

                for (i, field) in fields.into_iter().enumerate() {
                    let id = field.safe_ident();
                    let id_dyn = Cons::from(Rc::new(format!("{}_dyn", field.safe_ident())));
                    let i = i.to_string();

                    t.push({
                        let mut t = Tokens::new();

                        push!(t, "var ", id_dyn, " = _data[", i, "];");

                        if field.is_optional() {
                            t.push({
                                let mut t = Tokens::new();
                                push!(t, field.ty.ty(), " ", id, " = null;");
                                push!(t, "if (", id_dyn, " != null) {");

                                t.nested({
                                    let mut t = toks!();

                                    let (d, e) = field.ty.decode(id_dyn)?;
                                    t.push_unless_empty(e);
                                    push!(t, id, " = ", d, ";");
                                    t
                                });

                                t.push("}");
                                t
                            });
                        } else {
                            t.push({
                                let (d, e) = field.ty.decode(id_dyn.clone())?;
                                let mut t = toks!();
                                t.push(AssertNotNull(id_dyn));
                                t.push_unless_empty(e);
                                push!(t, "final ", field.ty.ty(), " ", id, " = ", d, ";");
                                t
                            });
                        }

                        t
                    });

                    vars.append(id);
                }

                push!(t, "return ", name, "(", vars.join(", "), ");");
                t.join_line_spacing()
            });

            t.join_line_spacing()
        });

        push!(t, "}");
        Ok(t)
    }

    /// Build a decode function for an interface.
    fn decode_interface_fn(
        &self,
        name: Cons<'el>,
        body: &'el RpInterfaceBody,
    ) -> Result<Tokens<'el, Dart<'el>>> {
        let mut t = toks!();

        push!(t, "static ", name, " decode(dynamic _dataDyn) {");

        t.push({
            let mut t = toks!();

            t.push({
                let mut t = toks!();
                t.push(AssertType(self.map_of_strings.clone(), "_dataDyn"));
                push!(t, self.map_of_strings, " _data = _dataDyn;");
                t
            });

            match body.sub_type_strategy {
                core::RpSubTypeStrategy::Tagged { ref tag, .. } => {
                    push!(t, "var tag = _data[", tag.as_str().quoted(), "];");

                    t.push({
                        let mut t = toks!();
                        push!(t, "switch (tag) {");

                        for s in &body.sub_types {
                            let name = self.convert_type_name(&s.name);
                            push!(t, "case ", s.name().quoted(), ":");
                            nested!(t, "return ", name, ".decode(_data);");
                        }

                        push!(t, "default:");
                        nested!(t, "throw 'bad tag: $tag';");

                        push!(t, "}");
                        t
                    });
                }
                core::RpSubTypeStrategy::Untagged => {
                    push!(t, "var keys = Set.of(_data.keys);");

                    for s in &body.sub_types {
                        let name = self.convert_type_name(&s.name);
                        let test: Tokens<'el, Dart<'el>> = s
                            .discriminating_fields()
                            .map(|f| f.name().quoted())
                            .collect();
                        let test = test.join(", ");

                        t.push({
                            let mut t = Tokens::new();
                            push!(t, "if (keys.containsAll(<String>[", test, "])) {");
                            nested!(t, "return ", name, ".decode(_data);");
                            push!(t, "}");
                            t
                        });
                    }
                }
            }

            t.join_line_spacing()
        });

        push!(t, "}");
        Ok(t)
    }

    /// Build an encode function.
    fn encode_fn(
        &self,
        name: Cons<'el>,
        fields: impl IntoIterator<Item = &'el Loc<RpField>>,
        tag: Option<&'el str>,
    ) -> Result<Tokens<'el, Dart<'el>>> {
        let mut t = Tokens::new();
        push!(t, self.map_of_strings, " encode() {");

        t.nested({
            let mut t = Tokens::new();

            push!(t, self.map_of_strings, " _data = Map();");

            if let Some(tag) = tag {
                push!(t, "_data[", tag.quoted(), "] = ", name.quoted(), ";");
            }

            for field in fields {
                let id = Cons::from(format!("this.{}", field.safe_ident()));
                let encoded = field.ty.encode(toks!(id.clone()))?;

                if field.is_optional() {
                    t.push({
                        let mut t = Tokens::new();
                        push!(t, "if (", id, " != null) {");
                        t.nested(toks!("_data[", field.name().quoted(), "] = ", encoded, ";"));
                        push!(t, "}");
                        t
                    });
                } else {
                    t.push(toks!("_data[", field.name().quoted(), "] = ", encoded, ";"));
                }
            }

            t.push("return _data;");

            t.join_line_spacing()
        });

        push!(t, "}");
        Ok(t)
    }

    /// Build an encode function to encode tuples.
    fn encode_tuple_fn(
        &self,
        fields: impl IntoIterator<Item = &'el Loc<RpField>>,
    ) -> Result<Tokens<'el, Dart<'el>>> {
        let mut t = Tokens::new();
        push!(t, self.list_of_dynamic, " encode() {");

        t.nested({
            let mut t = Tokens::new();

            push!(t, self.list_of_dynamic, " _data = List();");

            for field in fields {
                let e = field.ty.encode(toks!("this.", field.safe_ident()))?;
                push!(t, "_data.add(", e, ");");
            }

            t.push("return _data;");

            t.join_line_spacing()
        });

        push!(t, "}");
        Ok(t)
    }

    /// Setup a constructor based on the number of fields.
    fn constructor(
        &self,
        name: Cons<'el>,
        fields: impl IntoIterator<Item = &'el Loc<RpField>>,
    ) -> Result<Tokens<'el, Dart<'el>>> {
        let mut args = toks!();

        for field in fields {
            args.append(toks!("this.", field.safe_ident()));
        }

        let mut t = Tokens::new();

        if !args.is_empty() {
            push!(t, name.clone(), "(");
            nested!(t, args.join(toks!(",", Element::PushSpacing)));
            push!(t, ");");
        }

        Ok(t)
    }
}

impl<'el> PackageProcessor<'el, DartFlavor, RpName> for Compiler<'el> {
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

    fn default_process(&self, _out: &mut Self::Out, _: &RpName) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        push!(t, "class ", name, "{");
        t.nested({
            let mut t = Tokens::new();
            t.push_unless_empty(self.type_fields(&body.fields)?);
            t.push_unless_empty(self.constructor(name.clone(), &body.fields)?);
            t.push(self.decode_tuple_fn(name.clone(), &body.fields)?);
            t.push(self.encode_tuple_fn(&body.fields)?);
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
                    push!(
                        fields,
                        field,
                        " = const ",
                        name,
                        "._new(",
                        string.quoted(),
                        ");"
                    );
                }
                core::RpVariantValue::Number(number) => {
                    push!(
                        fields,
                        field,
                        " = const ",
                        name,
                        "._new(",
                        number.to_string(),
                        ");"
                    );
                }
            }
        }

        let f = "_value";

        let decode_fn = {
            let mut t = Tokens::new();
            push!(t, "static ", name, " decode(dynamic data) {");

            let ty = body.enum_type.ty();

            t.nested({
                let mut t = Tokens::new();

                t.push(AssertType(ty.clone(), "data"));

                t.push({
                    let mut t = Tokens::new();

                    push!(t, "switch (data as ", ty, ") {");

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
            push!(t, body.enum_type.ty(), " encode() {");
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

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));
        t.push(toks!["class ", name.clone(), " {"]);

        // fields
        t.nested({
            let mut t = Tokens::new();
            t.push_unless_empty(self.type_fields(&body.fields)?);
            t.push_unless_empty(self.constructor(name.clone(), &body.fields)?);
            t.push(self.decode_fn(name.clone(), &body.fields)?);
            t.push(self.encode_fn(name.clone(), &body.fields, None)?);
            t.join_line_spacing()
        });

        t.push("}");

        out.0.push(t);

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody) -> Result<()> {
        let name = self.convert_type_name(&body.name);

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(&body.comment));

        t.push({
            let mut t = toks!();
            t.push(toks!["abstract class ", name.clone(), " {"]);
            t.nested({
                let mut t = toks!();
                t.push(self.decode_interface_fn(name.clone(), body)?);
                push!(t, self.map_of_strings, " encode();");
                t.push_unless_empty(code!(&body.codes, core::RpContext::Dart));
                t.join_line_spacing()
            });
            t.push("}");
            t
        });

        let sup = name;

        for s in &body.sub_types {
            let name = self.convert_type_name(&s.name);
            let fields = body.fields.iter().chain(s.fields.iter());

            t.push({
                let mut t = Tokens::new();

                t.push_unless_empty(Comments(&s.comment));

                push!(t, "class ", name, " extends ", sup, " {");

                t.nested({
                    let mut t = Tokens::new();
                    t.push_unless_empty(self.type_fields(fields.clone())?);
                    t.push_unless_empty(self.constructor(name.clone(), fields.clone())?);
                    t.push(self.decode_fn(name.clone(), fields.clone())?);

                    match body.sub_type_strategy {
                        core::RpSubTypeStrategy::Tagged { ref tag, .. } => {
                            t.push(self.encode_fn(
                                s.name().into(),
                                fields.clone(),
                                Some(tag.as_str()),
                            )?);
                        }
                        core::RpSubTypeStrategy::Untagged => {
                            t.push(self.encode_fn(s.name().into(), fields.clone(), None)?);
                        }
                    }

                    t.push_unless_empty(code!(&s.codes, core::RpContext::Dart));
                    t.join_line_spacing()
                });

                t.push("}");

                t
            });
        }

        out.0.push(t.join_line_spacing());
        Ok(())
    }

    fn process_service(&self, _: &mut Self::Out, _: &'el RpServiceBody) -> Result<()> {
        Ok(())
    }
}
