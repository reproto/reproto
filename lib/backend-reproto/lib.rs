#[macro_use]
extern crate genco;
#[macro_use]
extern crate log;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_lexer as lexer;
extern crate reproto_manifest as manifest;
extern crate reproto_trans as trans;
extern crate toml;

use core::{Context, RelativePathBuf, RpDecl, RpField, RpInterfaceBody, RpSubTypeStrategy,
           RpTupleBody, RpTypeBody, DEFAULT_TAG};
use core::errors::Result;
use genco::{Custom, Formatter, IoFmt, Quoted, Tokens, WriteTokens};
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use std::fmt::{self, Write};
use std::path::Path;
use std::rc::Rc;
use trans::Environment;

#[derive(Default)]
pub struct ReprotoLang;

impl Lang for ReprotoLang {
    type Module = ReprotoModule;

    fn comment(input: &str) -> Option<String> {
        Some(format!("//{}", input.to_string()))
    }
}

#[derive(Debug)]
pub enum ReprotoModule {
}

impl TryFromToml for ReprotoModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

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

/// Compile to a reproto manifest.
pub fn compile(ctx: Rc<Context>, env: Environment, manifest: Manifest<ReprotoLang>) -> Result<()> {
    let handle = ctx.filesystem(manifest.output.as_ref().map(AsRef::as_ref))?;

    let root = RelativePathBuf::from(".");

    for (package, file) in env.for_each_file() {
        let mut path = package
            .package
            .parts
            .iter()
            .fold(root.clone(), |path, part| path.join(part));

        let parent = path.parent()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| root.clone());

        if !handle.is_dir(&parent) {
            debug!("+dir: {}", parent.display());
            handle.create_dir_all(&parent)?;
        }

        let path = if let Some(version) = package.version.as_ref() {
            let stem = path.file_stem()
                .ok_or_else(|| format!("Missing file stem: {}", path.display()))?;

            let file_name = format!("{}-{}.reproto", stem, version);
            path.with_file_name(file_name)
        } else {
            path.with_extension("reproto")
        };

        let mut body = Tokens::new();

        for decl in &file.decls {
            body.push(format(decl)?);
        }

        let body = body.join_line_spacing();

        debug!("+file: {}", path.display());
        IoFmt(&mut handle.create(&path)?).write_file(body, &mut ())?;
    }

    Ok(())
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
