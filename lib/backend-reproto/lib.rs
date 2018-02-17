#[macro_use]
extern crate genco;
extern crate reproto_core as core;
extern crate reproto_lexer as lexer;

use core::{RpDecl, RpField, RpInterfaceBody, RpSubTypeStrategy, RpTupleBody, RpTypeBody,
           DEFAULT_TAG};
use core::errors::Result;
use genco::{Custom, Formatter, Quoted, Tokens};
use std::fmt::{self, Write};

#[derive(Clone)]
pub enum Reproto {
}

impl Custom for Reproto {
    type Extra = ();

    fn quote_string(out: &mut Formatter, input: &str) -> fmt::Result {
        out.write_char('"')?;

        for c in input.chars() {
            match c {
                '\t' => out.write_str("\\t")?,
                '\u{0007}' => out.write_str("\\b")?,
                '\n' => out.write_str("\\n")?,
                '\r' => out.write_str("\\r")?,
                '\u{0014}' => out.write_str("\\f")?,
                '\'' => out.write_str("\\'")?,
                '"' => out.write_str("\\\"")?,
                '\\' => out.write_str("\\\\")?,
                c => out.write_char(c)?,
            }
        }

        out.write_char('"')?;

        Ok(())
    }
}

/// Format a single declaration as a reproto specification.
pub fn format<'el>(decl: &'el RpDecl) -> Result<Tokens<'el, Reproto>> {
    let result = match *decl {
        RpDecl::Type(ref type_) => format_type(type_),
        RpDecl::Interface(ref interface) => format_interface(interface),
        RpDecl::Tuple(ref tuple) => format_tuple(tuple),
        ref decl => return Err(format!("Unsupported declaration: {:?}", decl).into()),
    };

    return result;

    fn format_type<'el>(body: &'el RpTypeBody) -> Result<Tokens<'el, Reproto>> {
        let mut tuple = Tokens::new();

        tuple.push(toks!["type ", body.local_name.as_str(), " {"]);

        tuple.nested({
            let mut t = Tokens::new();

            for f in &body.fields {
                t.push(format_field(f)?);
            }

            for d in &body.decls {
                t.push(format(d)?);
            }

            t.join_line_spacing()
        });

        tuple.push("}");

        Ok(tuple)
    }

    fn format_interface<'el>(body: &'el RpInterfaceBody) -> Result<Tokens<'el, Reproto>> {
        let mut interface = Tokens::new();

        match body.sub_type_strategy {
            RpSubTypeStrategy::Tagged { ref tag, .. } => {
                if tag != DEFAULT_TAG {
                    interface.push(toks![
                        "#[type_info(strategy = ",
                        "tagged".quoted(),
                        ", tag = ",
                        tag.as_str().quoted(),
                        ")]"
                    ]);
                }
            }
        }

        interface.push(toks!["interface ", body.local_name.as_str(), " {"]);

        interface.nested({
            let mut t = Tokens::new();

            for sub_type in body.sub_types.values() {
                t.push({
                    let mut t = Tokens::new();

                    if let Some(ref alias) = sub_type.sub_type_name {
                        t.push(toks![
                            sub_type.local_name.as_str(),
                            " as ",
                            alias.as_str().quoted(),
                            " {"
                        ]);
                    } else {
                        t.push(toks![sub_type.local_name.as_str(), " {"]);
                    }

                    t.nested({
                        let mut t = Tokens::new();

                        for f in &sub_type.fields {
                            t.push(format_field(f)?);
                        }

                        for d in &sub_type.decls {
                            t.push(format(d)?);
                        }

                        t.join_line_spacing()
                    });

                    t.push("}");

                    t
                });
            }

            for d in &body.decls {
                t.push(format(d)?);
            }

            t.join_line_spacing()
        });

        interface.push("}");

        Ok(interface)
    }

    fn format_tuple<'el>(body: &'el RpTupleBody) -> Result<Tokens<'el, Reproto>> {
        let mut tuple = Tokens::new();

        tuple.push(toks!["tuple ", body.local_name.as_str(), " {"]);

        tuple.nested({
            let mut t = Tokens::new();

            for f in &body.fields {
                t.push(format_field(f)?);
            }

            for d in &body.decls {
                t.push(format(d)?);
            }

            t.join_line_spacing()
        });

        tuple.push("}");

        Ok(tuple)
    }

    fn format_field<'el>(field: &'el RpField) -> Result<Tokens<'el, Reproto>> {
        let mut t = Tokens::new();

        for line in &field.comment {
            if line.is_empty() {
                t.push("///");
            } else {
                t.push(toks!["/// ", line.as_str()]);
            }
        }

        let field_name = field.name.as_str();

        let field_name = match lexer::match_keyword(field_name) {
            Some(token) => token
                .keyword_safe()
                .ok_or_else(|| format!("keyword does not have a safe variant: {}", field_name))?,
            None => field_name,
        };

        if field.is_optional() {
            t.push(toks![field_name, "?: ", field.ty.to_string()]);
        } else {
            t.push(toks![field_name, ": ", field.ty.to_string()]);
        }

        if let Some(ref field_as) = field.field_as {
            t.extend(toks![" as ", field_as.as_str().quoted()]);
        }

        t.append(";");

        Ok(t)
    }
}
