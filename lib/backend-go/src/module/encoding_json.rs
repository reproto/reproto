//! encoding/json module for Go

use backend::Initializer;
use core;
use core::errors::{Error, Result};
use flavored::{GoName, RpEnumBody, RpInterfaceBody, RpSubType, RpTupleBody};
use genco::go::{imported, Go};
use genco::{Quoted, Tokens};
use std::rc::Rc;
use {
    EnumAdded, EnumCodegen, FieldAdded, FieldCodegen, InterfaceAdded, InterfaceCodegen, Options,
    TupleAdded, TupleCodegen,
};

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        let codegen = Rc::new(Codegen::new());
        options.field_gens.push(Box::new(codegen.clone()));
        options.enum_gens.push(Box::new(codegen.clone()));
        options.tuple_gens.push(Box::new(codegen.clone()));
        options.interface_gens.push(Box::new(codegen.clone()));
        Ok(())
    }
}

struct Codegen {
    new_error: Go<'static>,
    unmarshal: Go<'static>,
    marshal: Go<'static>,
    raw_message: Go<'static>,
}

impl Codegen {
    pub fn new() -> Codegen {
        Self {
            new_error: imported("errors", "New"),
            unmarshal: imported("encoding/json", "Unmarshal"),
            marshal: imported("encoding/json", "Marshal"),
            raw_message: imported("encoding/json", "RawMessage"),
        }
    }
}

impl FieldCodegen for Codegen {
    fn generate(&self, e: FieldAdded) -> Result<()> {
        let FieldAdded { tags, field, .. } = e;

        tags.push_str("json", field.name());

        if field.is_optional() {
            tags.push_str("json", "omitempty");
        }

        return Ok(());
    }
}

impl EnumCodegen for Codegen {
    fn generate(&self, e: EnumAdded) -> Result<()> {
        let EnumAdded {
            container,
            name,
            body,
            ..
        } = e;

        container.push(unmarshal_json(self, name, body));
        container.push(marshal_json(self, name, body));

        return Ok(());

        fn unmarshal_json<'el>(
            c: &Codegen,
            name: &'el GoName,
            body: &'el RpEnumBody,
        ) -> Tokens<'el, Go<'el>> {
            let mut t = Tokens::new();

            push!(t, "func (this *", name, ") UnmarshalJSON(b []byte) error {");

            t.nested({
                let mut t = Tokens::new();

                push!(t, "var s ", body.enum_type);

                t.push_into(|t| {
                    push!(t, "if err := ", c.unmarshal, "(b, &s); err != nil {");
                    nested!(t, "return err");
                    push!(t, "}");
                });

                t.push_into(|t| {
                    t.push("switch s {");

                    match body.variants {
                        core::RpVariants::String { ref variants } => for v in variants {
                            t.push_into(|t| {
                                push!(t, "case ", v.value.as_str().quoted(), ":");
                                nested!(t, "*this = ", name, "_", v.ident.as_str());
                            });
                        },
                        core::RpVariants::Number { ref variants } => for v in variants {
                            t.push_into(|t| {
                                push!(t, "case ", v.value.to_string(), ":");
                                nested!(t, "*this = ", name, "_", v.ident.as_str());
                            });
                        },
                    }

                    t.push_into(|t| {
                        push!(t, "default:");
                        nested!(t, "return ", c.new_error, "(", "bad value".quoted(), ")");
                    });

                    t.push("}");
                });

                t.push("return nil");

                t.join_line_spacing()
            });

            t.push("}");

            t
        }

        fn marshal_json<'el>(
            c: &Codegen,
            name: &'el GoName,
            body: &'el RpEnumBody,
        ) -> Tokens<'el, Go<'el>> {
            let mut t = Tokens::new();

            push!(t, "func (this ", name, ") MarshalJSON() ([]byte, error) {");

            t.nested({
                let mut t = Tokens::new();

                push!(t, "var s ", body.enum_type);

                t.push_into(|t| {
                    t.push("switch this {");

                    match body.variants {
                        core::RpVariants::String { ref variants } => for v in variants {
                            t.push_into(|t| {
                                t.push(toks!["case ", name, "_", v.ident.as_str(), ":"]);
                                t.nested(toks!["s = ", v.value.as_str().quoted()]);
                            });
                        },
                        core::RpVariants::Number { ref variants } => for v in variants {
                            t.push_into(|t| {
                                t.push(toks!["case ", name, "_", v.ident.as_str(), ":"]);
                                t.nested(toks!["s = ", v.value.to_string()]);
                            });
                        },
                    }

                    t.push_into(|t| {
                        push!(t, "default:");
                        nested!(
                            t,
                            "return nil, ",
                            c.new_error,
                            "(",
                            "bad value".quoted(),
                            ")"
                        );
                    });

                    t.push("}");
                });

                push!(t, "return ", c.marshal, "(s)");

                t.join_line_spacing()
            });

            t.push("}");

            t
        }
    }
}

impl TupleCodegen for Codegen {
    fn generate(&self, e: TupleAdded) -> Result<()> {
        let TupleAdded {
            container,
            name,
            body,
            ..
        } = e;

        container.push(unmarshal_json(self, name, body)?);
        container.push(marshal_json(self, name, body)?);

        return Ok(());

        fn unmarshal_json<'el>(
            c: &Codegen,
            name: &'el GoName,
            body: &'el RpTupleBody,
        ) -> Result<Tokens<'el, Go<'el>>> {
            let mut t = Tokens::new();

            t.try_push_into::<Error, _>(|t| {
                push!(t, "func (this *", name, ") UnmarshalJSON(b []byte) error {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push(toks!["var array []", c.raw_message.clone()]);

                    t.push_into(|t| {
                        t.push(toks![
                            "if err := ",
                            c.unmarshal.clone(),
                            "(b, &array); err != nil {"
                        ]);
                        t.nested("return err");
                        t.push("}");
                    });

                    t.push({
                        let mut t = Tokens::new();

                        for (i, f) in body.fields.iter().enumerate() {
                            let a = toks!["array[", i.to_string(), "]"];

                            let var = f.safe_ident();

                            t.push({
                                let mut t = Tokens::new();

                                t.push_into(|t| {
                                    t.push(toks!["var ", var.clone(), " ", f.ty.clone()]);

                                    t.push_into(|t| {
                                        t.push(toks![
                                            "if err := ",
                                            c.unmarshal.clone(),
                                            "(",
                                            a,
                                            ", &",
                                            var.clone(),
                                            "); err != nil {"
                                        ]);
                                        t.nested("return err");
                                        t.push("}");
                                    });

                                    t.push(toks!["this.", f.safe_ident(), " = ", var.clone()]);
                                });

                                t.join_line_spacing()
                            });
                        }

                        t.join_line_spacing()
                    });

                    t.push("return nil");

                    t.join_line_spacing()
                });

                t.push("}");

                Ok(())
            })?;

            Ok(t)
        }

        fn marshal_json<'el>(
            c: &Codegen,
            name: &'el GoName,
            body: &'el RpTupleBody,
        ) -> Result<Tokens<'el, Go<'el>>> {
            let mut t = Tokens::new();

            push!(t, "func (this ", name, ") MarshalJSON() ([]byte, error) {");

            t.nested({
                let mut t = Tokens::new();

                push!(t, "var array []", c.raw_message);

                t.push({
                    let mut t = Tokens::new();

                    for f in &body.fields {
                        let var = f.safe_ident();

                        t.push({
                            let mut t = Tokens::new();
                            let ident = toks!["this.", f.safe_ident()];

                            push!(t, var, ", err := ", c.marshal, "(", ident, ")");

                            t.push_into(|t| {
                                t.push("if err != nil {");
                                t.nested("return nil, err");
                                t.push("}");
                            });

                            push!(t, "array = append(array, ", var, ")");

                            t.join_line_spacing()
                        });
                    }

                    t.join_line_spacing()
                });

                push!(t, "return ", c.marshal, "(array)");

                t.join_line_spacing()
            });

            t.push("}");

            Ok(t)
        }
    }
}

impl InterfaceCodegen for Codegen {
    fn generate(&self, e: InterfaceAdded) -> Result<()> {
        let InterfaceAdded {
            container,
            name,
            body,
            ..
        } = e;

        container.push(unmarshal_json(self, name, body)?);
        container.push(marshal_json(self, name, body)?);

        return Ok(());

        fn unmarshal_json<'el>(
            c: &Codegen,
            name: &'el GoName,
            body: &'el RpInterfaceBody,
        ) -> Result<Tokens<'el, Go<'el>>> {
            let mut t = Tokens::new();

            t.try_push_into::<Error, _>(|t| {
                push!(t, "func (this *", name, ") UnmarshalJSON(b []byte) error {");

                match body.sub_type_strategy {
                    core::RpSubTypeStrategy::Tagged { ref tag } => {
                        t.nested(unmarshal_tagged(c, body, tag)?);
                    }
                    core::RpSubTypeStrategy::Untagged => {
                        t.nested(unmarshal_untagged(c, body)?);
                    }
                }

                push!(t, "}");
                Ok(())
            })?;

            return Ok(t);

            fn unmarshal_sub_type<'el>(
                c: &Codegen,
                sub_type: &'el RpSubType,
            ) -> Tokens<'el, Go<'el>> {
                let mut t = Tokens::new();

                push!(t, "sub := ", &sub_type.name, "{}");

                t.push_into(|t| {
                    push!(t, "if err = ", c.unmarshal, "(b, &sub); err != nil {");
                    nested!(t, "return err");
                    push!(t, "}");
                });

                t.push_into(|t| {
                    push!(t, "this.Value = &sub");
                    push!(t, "return nil");
                });

                t.join_line_spacing()
            }

            /// Unmarshal the envelope and extract the type field.
            fn unmarshal_tagged<'el>(
                c: &Codegen,
                body: &'el RpInterfaceBody,
                tag: &'el str,
            ) -> Result<Tokens<'el, Go<'el>>> {
                let mut t = Tokens::new();

                t.push_into(|t| {
                    push!(t, "var err error");
                    push!(t, "var ok bool");
                    push!(t, "env := make(map[string]", c.raw_message, ")");
                });

                t.push_into(|t| {
                    push!(t, "if err := ", c.unmarshal, "(b, &env); err != nil {");
                    nested!(t, "return err");
                    push!(t, "}");
                });

                push!(t, "var raw_tag ", c.raw_message);

                t.push_into(|t| {
                    push!(t, "if raw_tag, ok = env[", tag.quoted(), "]; !ok {");
                    nested!(t, "return ", c.new_error, "(", "missing tag".quoted(), ")");
                    push!(t, "}");
                });

                push!(t, "var tag string");

                t.push_into(|t| {
                    push!(t, "if err = ", c.unmarshal, "(raw_tag, &tag); err != nil {");
                    nested!(t, "return err");
                    push!(t, "}");
                });

                t.try_push_into::<Error, _>(|t| {
                    push!(t, "switch (tag) {");

                    for sub_type in &body.sub_types {
                        push!(t, "case ", sub_type.name().quoted(), ":");
                        t.nested(unmarshal_sub_type(c, sub_type));
                    }

                    push!(t, "default:");
                    nested!(t, "return ", c.new_error, "(", "bad tag".quoted(), ")");

                    push!(t, "}");
                    Ok(())
                })?;

                Ok(t.join_line_spacing())
            }

            fn unmarshal_untagged<'el>(
                c: &Codegen,
                body: &'el RpInterfaceBody,
            ) -> Result<Tokens<'el, Go<'el>>> {
                let mut t = Tokens::new();

                t.push_into(|t| {
                    push!(t, "var err error");
                    push!(t, "env := make(map[string]", c.raw_message, ")");
                });

                t.push_into(|t| {
                    push!(t, "if err := ", c.unmarshal, "(b, &env); err != nil {");
                    nested!(t, "return err");
                    push!(t, "}");
                });

                push!(t, "keys := make(map[string]bool)");

                t.push_into(|t| {
                    push!(t, "for k := range env {");
                    nested!(t, "keys[k] = true");
                    push!(t, "}");
                });

                push!(t, "var all bool");

                for sub_type in &body.sub_types {
                    t.push_into(|t| {
                        push!(t, "all = true");

                        let mut required = Tokens::new();

                        for f in sub_type.discriminating_fields() {
                            required.append(f.name().quoted());
                        }

                        let required = toks!["[]string{", required.join(", "), "}"];

                        push!(t, "for _, k := range(", required, ") {");

                        t.nested_into(|t| {
                            push!(t, "if _, all = keys[k]; !all {");
                            nested!(t, "break");
                            push!(t, "}");
                        });

                        push!(t, "}");
                    });

                    t.push_into(|t| {
                        push!(t, "if all {");
                        t.nested(unmarshal_sub_type(c, sub_type));
                        push!(t, "}");
                    });
                }

                push!(
                    t,
                    "return ",
                    c.new_error,
                    "(",
                    "no combination of fields found".quoted(),
                    ")"
                );
                Ok(t.join_line_spacing())
            }
        }

        fn marshal_json<'el>(
            c: &Codegen,
            name: &'el GoName,
            body: &'el RpInterfaceBody,
        ) -> Result<Tokens<'el, Go<'el>>> {
            let mut t = Tokens::new();

            t.push({
                let mut t = Tokens::new();

                push!(t, "func (this ", name, ") MarshalJSON() ([]byte, error) {");

                match body.sub_type_strategy {
                    core::RpSubTypeStrategy::Tagged { ref tag } => {
                        t.nested(marshal_tagged(c, body, tag)?);
                    }
                    core::RpSubTypeStrategy::Untagged => {
                        t.nested(marshal_untagged(c, body)?);
                    }
                }

                push!(t, "}");

                t
            });

            return Ok(t);

            /// Marshal the envelope and extract the type field.
            fn marshal_tagged<'el>(
                c: &Codegen,
                body: &'el RpInterfaceBody,
                tag: &'el str,
            ) -> Result<Tokens<'el, Go<'el>>> {
                let mut t = Tokens::new();

                t.push_into(|t| {
                    push!(t, "var b []byte");
                    push!(t, "var err error");
                    push!(t, "env := make(map[string]", c.raw_message, ")");
                });

                t.push_into(|t| {
                    push!(t, "switch v := this.Value.(type) {");

                    for sub_type in &body.sub_types {
                        t.push(marshal_tagged_sub_type(c, sub_type, tag));
                    }

                    t.push_into(|t| {
                        let m = format!("{}: no sub-type set", body.name);
                        let error = toks!(c.new_error.clone(), "(", m.quoted(), ")");
                        push!(t, "default:");
                        nested!(t, "return nil, ", error);
                    });

                    push!(t, "}");
                });

                return Ok(t.join_line_spacing());
            }

            /// Marshal the sub-type immediately.
            fn marshal_untagged<'el>(
                c: &Codegen,
                body: &'el RpInterfaceBody,
            ) -> Result<Tokens<'el, Go<'el>>> {
                let mut t = Tokens::new();

                t.push({
                    let mut t = Tokens::new();

                    t.push_into(|t| {
                        push!(t, "switch v := this.Value.(type) {");

                        for sub_type in &body.sub_types {
                            push!(t, "case *", &sub_type.name, ":");
                            nested!(t, "return ", c.marshal, "(v)");
                        }

                        t.push_into(|t| {
                            let m = format!("{}: no sub-type set", body.name);
                            let error = toks!(c.new_error.clone(), "(", m.quoted(), ")");

                            push!(t, "default:");
                            nested!(t, "return nil, ", error);
                        });

                        push!(t, "}");
                    });

                    t.join_line_spacing()
                });

                return Ok(t.join_line_spacing());
            }

            fn marshal_tagged_sub_type<'el>(
                c: &Codegen,
                sub_type: &'el RpSubType,
                tag: &'el str,
            ) -> Tokens<'el, Go<'el>> {
                let mut t = Tokens::new();

                push!(t, "case *", &sub_type.name, ":");

                t.nested({
                    let mut t = Tokens::new();

                    t.push_into(|t| {
                        push!(t, "if b, err = ", c.marshal, "(v); err != nil {");
                        nested!(t, "return nil, err");
                        push!(t, "}");
                    });

                    t.push_into(|t| {
                        push!(t, "if err = ", c.unmarshal, "(b, &env); err != nil {");
                        nested!(t, "return nil, err");
                        push!(t, "}");
                    });

                    let o = toks!("env[", tag.quoted(), "]");

                    t.push_into(|t| {
                        let m = toks!(c.marshal.clone(), "(", sub_type.name().quoted(), ")");

                        push!(t, "if ", o, ", err = ", m, "; err != nil {");
                        nested!(t, "return nil, err");
                        push!(t, "}");
                    });

                    push!(t, "return ", c.marshal, "(env)");

                    t.join_line_spacing()
                });

                t
            }
        }
    }
}
