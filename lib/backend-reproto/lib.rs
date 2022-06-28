use genco::fmt;
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr};
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::flavored::*;
use reproto_core::{CoreFlavor, Handle, RelativePathBuf, Spanned, DEFAULT_TAG};
use std::any::Any;
use std::fmt::Write as _;
use std::path::Path;
use trans::Session;

pub enum Interior<'a> {
    Field(&'a Spanned<RpField>),
    Decl(&'a RpDecl),
    SubType(&'a Spanned<RpSubType>),
}

impl FormatInto<Reproto> for Interior<'_> {
    fn format_into(self, t: &mut Tokens<Reproto>) {
        match self {
            Self::Field(f) => {
                format_field(t, f);
                t.append(";");
            }
            Self::Decl(d) => format(t, d),
            Self::SubType(sub_type) => {
                let interior = sub_type
                    .fields
                    .iter()
                    .map(Interior::Field)
                    .chain(sub_type.decls.iter().map(Interior::Decl));

                quote_in! { *t =>
                    #(if let Some(ref alias) = sub_type.sub_type_name {
                        #(sub_type.ident.as_str()) as #(quoted(alias.as_str()))
                    } else {
                        #(sub_type.ident.as_str())
                    }) {
                        #(for i in interior join (#<line>) => #i)
                    }
                }
            }
        }
    }
}

pub struct Comments<I>(I);

impl<I> FormatInto<Reproto> for Comments<I>
where
    I: IntoIterator,
    I::Item: Into<ItemStr>,
{
    fn format_into(self, t: &mut Tokens<Reproto>) {
        for line in self.0 {
            let line = line.into();

            t.push();

            if line.is_empty() {
                t.append("///");
            } else {
                t.append("///");
                t.space();
                t.append(line);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReprotoLang;

impl Lang for ReprotoLang {
    manifest::lang_base!(ReprotoModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("//{}", input.to_string()))
    }
}

#[derive(Debug)]
pub enum ReprotoModule {}

impl TryFromToml for ReprotoModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reproto(());

impl genco::lang::Lang for Reproto {
    type Config = ();
    type Format = ();
    type Item = ();

    fn write_quoted(out: &mut fmt::Formatter, input: &str) -> fmt::Result {
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

        Ok(())
    }
}

/// Compile to a reproto manifest.
fn compile(handle: &dyn Handle, env: Session<CoreFlavor>, _manifest: Manifest) -> Result<()> {
    let env = env.translate_default()?;

    let root = RelativePathBuf::from(".");

    for (package, file) in env.for_each_file() {
        let path = package
            .package
            .parts()
            .fold(root.clone(), |path, part| path.join(part));

        let parent = path
            .parent()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| root.clone());

        if !handle.is_dir(&parent) {
            log::debug!("+dir: {}", parent);
            handle.create_dir_all(&parent)?;
        }

        let path = if let Some(version) = package.version.as_ref() {
            let stem = path
                .file_stem()
                .ok_or_else(|| format!("Missing file stem: {}", path))?;

            let file_name = format!("{}-{}.reproto", stem, version);
            path.with_file_name(file_name)
        } else {
            path.with_extension("reproto")
        };

        let mut body = Tokens::new();

        for decl in &file.decls {
            format(&mut body, decl);
            body.line();
        }

        log::debug!("+file: {}", path);
        body.line();

        let mut w = fmt::IoWriter::new(handle.create(&path)?);
        let fmt = fmt::Config::from_lang::<Reproto>().with_indentation(fmt::Indentation::Space(2));

        body.format_file(&mut w.as_formatter(&fmt), &())?;
    }

    Ok(())
}

/// Format a single declaration as a reproto specification.
pub fn format(out: &mut Tokens<Reproto>, decl: &RpDecl) {
    match decl {
        RpDecl::Type(type_) => format_type(out, type_),
        RpDecl::Interface(interface) => format_interface(out, interface),
        RpDecl::Tuple(tuple) => format_tuple(out, tuple),
        RpDecl::Enum(en) => format_enum(out, en),
        RpDecl::Service(service) => format_service(out, service),
    }
}

fn format_type(out: &mut Tokens<Reproto>, body: &RpTypeBody) {
    let interior = body
        .fields
        .iter()
        .map(Interior::Field)
        .chain(body.decls.iter().map(Interior::Decl));

    quote_in! { *out =>
        #(Comments(&body.comment))
        type #(body.ident.as_str()) {
            #(for i in interior join (#<line>) => #i)
        }
    }
}

fn format_interface(out: &mut Tokens<Reproto>, body: &RpInterfaceBody) {
    let interior = body
        .sub_types
        .iter()
        .map(Interior::SubType)
        .chain(body.decls.iter().map(Interior::Decl));

    quote_in! { *out =>
        #(match &body.sub_type_strategy {
            RpSubTypeStrategy::Tagged { tag, .. } if tag != DEFAULT_TAG => {
                #[type_info(strategy = "tagged", tag = #(quoted(tag.as_str())))]
            }
            RpSubTypeStrategy::Untagged => {
                #[type_info(strategy = "untagged")]
            }
            _ => {}
        })
        #(Comments(&body.comment))
        interface #(body.ident.as_str()) {
            #(for i in interior join (#<line>) => #i)
        }
    }
}

fn format_tuple(out: &mut Tokens<Reproto>, body: &RpTupleBody) {
    let interior = body
        .fields
        .iter()
        .map(Interior::Field)
        .chain(body.decls.iter().map(Interior::Decl));

    quote_in! { *out =>
        #(Comments(&body.comment))
        tuple #(body.ident.as_str()) {
            #(for i in interior join (#<line>) => #i)
        }
    }
}

fn format_enum(out: &mut Tokens<Reproto>, body: &RpEnumBody) {
    quote_in! { *out =>
        #(Comments(&body.comment))
        enum #(&body.ident) as #(body.enum_type.to_string()) {
            #(for v in &body.variants join (#<line>) =>
                #(ref out => format_variant(out, v))
            )
        }
    }
}

fn format_service(out: &mut Tokens<Reproto>, body: &RpServiceBody) {
    quote_in! { *out =>
        #(Comments(&body.comment))
        service #(body.ident.as_str()) {
            #(for e in &body.endpoints join (#<line>) =>
                #(Comments(&e.comment))
                #(ref out => format_endpoint(out, e))
            )
        }
    }

    return;

    fn format_endpoint(out: &mut Tokens<Reproto>, e: &RpEndpoint) {
        quote_in! { *out =>
            #(e.ident.as_str())(#(for a in &e.arguments join (, ) =>
                #(a.ident.as_str()): #(a.channel.to_string())
            ))
        }
    }
}

fn format_field(out: &mut Tokens<Reproto>, field: &RpField) {
    let field_name = field.safe_ident();

    let field_name = match lexer::match_keyword(field_name) {
        Some(token) => token.keyword_safe(),
        None => field_name,
    };

    quote_in! { *out =>
        #(Comments(&field.comment))
        #(if field.is_optional() {
            #(field_name)?: #(&field.ty.to_string())
        } else {
            #(field_name): #(field.ty.to_string())
        })#(if let Some(ref field_as) = field.field_as {
            #<space>as #(quoted(field_as.as_str()))
        })
    }
}

fn format_variant(out: &mut Tokens<Reproto>, variant: RpVariantRef<'_>) {
    quote_in! { *out =>
        #(Comments(variant.comment))
        #(variant.ident()) as #(match variant.value {
            RpVariantValue::String(string) => {
                #(quoted(string))
            }
            RpVariantValue::Number(number) => {
                #(number.to_string())
            }
        });
    }
}
