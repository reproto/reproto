//! Processor trait.

use super::{DOC_CSS_NAME, NORMALIZE_CSS_NAME};
use crate::doc_builder::DocBuilder;
use crate::escape::Escape;
use crate::macros::FormatAttribute;
use crate::rendering::markdown_to_html;
use reproto_core::errors::Result;
use reproto_core::flavored::*;
use reproto_core::{AsPackage, CoreFlavor, Spanned};
use std::ops::DerefMut;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;
use trans::Translated;

pub trait Processor<'session> {
    /// Access the current builder.
    fn out(&self) -> ::std::cell::RefMut<DocBuilder<'session>>;

    /// Access the current session.
    fn session(&self) -> &'session Translated<CoreFlavor>;

    /// Path to root.
    fn root(&self) -> &'session str;

    /// Process the given request.
    fn process(self) -> Result<()>;

    /// Syntax theme.
    fn syntax(&self) -> (&'session Theme, &'session SyntaxSet);

    fn current_package(&self) -> Option<&'session RpVersionedPackage> {
        None
    }

    /// Generate a type URL.
    fn type_url(&self, name: &RpName) -> Result<String> {
        let reg = self.session().lookup(name)?;

        let (fragment, path) = match *reg {
            RpReg::EnumVariant | RpReg::SubType => {
                let fragment = format!("#{}", name.path.clone().join("_"));

                let path: Vec<_> = name
                    .path
                    .iter()
                    .cloned()
                    .take(name.path.len() - 1)
                    .collect();

                (fragment, path)
            }
            _ => {
                let fragment = "".to_string();
                (fragment, name.path.clone())
            }
        };

        if let Some(_) = name.prefix {
            let package_path = name.package.try_as_package()?.join("/");

            return Ok(format!(
                "{}/{}/{}.{}.html{}",
                self.root(),
                package_path,
                reg,
                path.join("."),
                fragment,
            ));
        }

        Ok(format!("{}.{}.html{}", reg, path.join("."), fragment))
    }

    fn markdown(&self, comment: &str) -> Result<()> {
        if !comment.is_empty() {
            let (theme, syntax_set) = self.syntax();
            markdown_to_html(self.out().deref_mut(), comment, theme, syntax_set)?;
        }

        Ok(())
    }

    fn doc<'a, I>(&self, comment: I) -> Result<()>
    where
        I: IntoIterator<Item = &'a String>,
    {
        let mut it = comment.into_iter().peekable();

        if it.peek().is_some() {
            let comment = it.map(ToOwned::to_owned).collect::<Vec<_>>();
            let comment = comment.join("\n");
            html!(self, div { class => "doc" } => {
                self.markdown(comment.as_str())?;
            });
        } else {
            html!(self, div { class => "missing-doc" } ~ Escape(""));
        }

        Ok(())
    }

    fn primitive(&self, name: &str) -> Result<()> {
        html!(self, span {class => format!("type-{} type-primitive", name)} ~ name);
        Ok(())
    }

    fn write_type(&self, ty: &RpType) -> Result<()> {
        write!(self.out(), "<span class=\"ty\">")?;

        match ty {
            RpType::Double => self.primitive("double")?,
            RpType::Float => self.primitive("float")?,
            RpType::Boolean => self.primitive("boolean")?,
            RpType::String(..) => self.primitive("string")?,
            RpType::DateTime => self.primitive("datetime")?,
            RpType::Bytes => self.primitive("bytes")?,
            RpType::Any => self.primitive("any")?,
            RpType::Number(number) => self.primitive(number.to_string().as_str())?,
            RpType::Name { name } => {
                html!(self, span {class => "type-rp-name"} => {
                    self.full_name_without_package(name)?;
                });
            }
            RpType::Array { inner } => {
                html!(self, span {class => "type-array"} => {
                    html!(self, span {class => "type-array-left"} ~ "[");
                    self.write_type(inner)?;
                    html!(self, span {class => "type-array-right"} ~ "]");
                });
            }
            RpType::Map { key, value } => {
                html!(self, span {class => "type-map"} => {
                    html!(self, span {class => "type-map-left"} ~ "{");
                    self.write_type(key)?;
                    html!(self, span {class => "type-map-sep"} ~ ":");
                    self.write_type(value)?;
                    html!(self, span {class => "type-map-right"} ~ "}");
                });
            }
        }

        write!(self.out(), "</span>")?;
        Ok(())
    }

    fn field_overview(&self, field: &RpField) -> Result<()> {
        let mut classes = vec!["field"];

        if field.is_optional() {
            classes.push("optional");
        } else {
            classes.push("required");
        }

        html!(self, h2 {class => "field-title"} => {
            html!(self, span {class => "field-key"} => {
                html!(self, a {href => format!("#field.{}", field.ident)} => {
                    html!(self, span {class => "field-id"} ~ Escape(&field.ident));
                });

                if field.is_optional() {
                    html!(self, span {class => "field-modifier"} ~ "?");
                }

                html!(self, span {} ~ ":");
            });

            self.write_type(&field.ty)?;

            if field.ident != field.name() {
                html!(self, span {class => "keyword"} ~ "as");
                html!(self, span {class => "field-name"} ~ Escape(field.name()));
            }
        });

        self.doc(field.comment.iter().take(1))?;
        Ok(())
    }

    fn fields_overview<'b, I>(&self, fields: I) -> Result<()>
    where
        I: IntoIterator<Item = &'b Spanned<RpField>>,
    {
        for field in fields {
            self.field_overview(field)?;
        }

        Ok(())
    }

    fn field(&self, field: &RpField) -> Result<()> {
        let mut classes = vec!["field"];

        if field.is_optional() {
            classes.push("optional");
        } else {
            classes.push("required");
        }

        html!(self, h2 {class => "field-title", id => format!("field.{}", field.ident)} => {
            html!(self, span {class => "kind"} ~ "field");

            html!(self, span {class => "field-key"} => {
                html!(self, span {class => "field-id"} ~ Escape(&field.ident));

                if field.is_optional() {
                    html!(self, span {class => "field-modifier"} ~ "?");
                }

                html!(self, span {} ~ ":");
            });

            self.write_type(&field.ty)?;

            if field.ident != field.name() {
                html!(self, span {class => "keyword"} ~ "as");
                html!(self, span {class => "field-name"} ~ Escape(field.name()));
            }
        });

        self.doc(&field.comment)?;

        Ok(())
    }

    fn fields<'b, I>(&self, fields: I) -> Result<()>
    where
        I: IntoIterator<Item = &'b Spanned<RpField>>,
    {
        for field in fields {
            self.field(field)?;
        }

        Ok(())
    }

    /// Render a nested declaration
    fn nested_decl_overview(&self, decl: &RpDecl) -> Result<()> {
        html!(self, h2 {class => "decl-title"} => {
            html!(self, span {class => "kind"} ~ decl.kind());
            self.full_name_without_package(&decl.name())?;
        });

        self.doc(decl.comment().iter().take(1))?;
        Ok(())
    }

    /// Render overview of nested declarations
    fn nested_decls_overview<'b, I>(&self, decls: I) -> Result<()>
    where
        I: IntoIterator<Item = &'b RpDecl>,
    {
        for decl in decls {
            self.nested_decl_overview(decl)?;
        }

        Ok(())
    }

    /// Render a nested declaration
    fn nested_decl(&self, decl: &RpDecl) -> Result<()> {
        html!(self, h2 {class => "decl-title"} => {
            html!(self, span {class => "kind"} ~ format!("nested {}", decl.kind()));
            self.full_name_without_package(&decl.name())?;
        });

        self.doc(decl.comment().iter().take(1))?;
        Ok(())
    }

    /// Render a set of nested declarations
    fn nested_decls<'b, I>(&self, decls: I) -> Result<()>
    where
        I: IntoIterator<Item = &'b RpDecl>,
    {
        for decl in decls {
            self.nested_decl(decl)?;
        }

        Ok(())
    }

    /// Write a section title.
    fn section_title(&self, kind: &str, name: &RpName) -> Result<()> {
        html!(self, h1 {class => "section-title"} => {
            html!(self, span {class => "kind"} ~ kind);
            self.full_name(name, Some(name))?;
        });

        Ok(())
    }

    /// Write a complete HTML document.
    fn write_doc<Body>(&self, body: Body) -> Result<()>
    where
        Body: FnOnce() -> Result<()>,
    {
        html!(self, html {} => {
            html!(self, head {} => {
                html!(@open self, meta {charset => "utf-8"});
                self.out().new_line()?;

                html!(@open self, meta {
                    name => "viewport",
                    content => "width=device-width, initial-scale=1.0"
                });
                self.out().new_line()?;

                html!(@open self, link {
                    rel => "stylesheet", type => "text/css",
                    href => format!("{}/{}", self.root(), NORMALIZE_CSS_NAME)
                });
                self.out().new_line()?;

                html!(@open self, link {
                    rel => "stylesheet", type => "text/css",
                    href => format!("{}/{}", self.root(), DOC_CSS_NAME)
                });
            });

            html!(self, body {} => {
                html!(self, div {class => "container"} => {
                    html!(self, nav {class => "top"} => {
                        html!(self, a {href => format!("{}/index.html", self.root())} ~ "Index");

                        if let Some(package) = self.current_package() {
                            let package_url = self.package_url(package);
                            html!(self, span {} ~ "&mdash;");
                            html!(self, a {href => package_url} ~ format!("Package: {}", package));
                        }
                    });

                    body()?;
                });
            });
        });

        Ok(())
    }

    fn package_url(&self, package: &RpVersionedPackage) -> String {
        let url = package.clone().to_package(|v| v.to_string()).join("/");
        format!("{}/{}/index.html", self.root(), url)
    }

    fn fragment_filter(url: &str) -> String {
        let mut bytes = [0u8; 4];
        let mut buffer = String::with_capacity(url.len());

        for c in url.chars() {
            let encode = match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => false,
                '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' => false,
                '-' | '.' | '_' | '~' | ':' | '@' | '/' | '?' => false,
                _ => true,
            };

            if encode {
                let result = c.encode_utf8(&mut bytes);

                for b in result.bytes() {
                    buffer.extend(format!("%{:X}", b).chars());
                }

                continue;
            }

            buffer.push(c);
        }

        buffer
    }

    /// Write the full path to a name.
    ///
    /// # Examples
    ///
    /// ```html
    /// <span class="name-part">Foo</span>
    /// <span class="name-sep">::</span>
    /// <span class="name-local">Bar</span>
    /// ```
    fn full_name(&self, name: &RpName, current: Option<&RpName>) -> Result<()> {
        /*let package_url = self.package_url(&name.package);
        html!(self, a {class => "name-package", href => package_url} ~ name.package.to_string());
        html!(self, span {class => "name-sep"} ~ "::");*/

        let mut it = name.path.iter();
        let local = it
            .next_back()
            .ok_or_else(|| "local part of name required")?;

        let mut path = Vec::new();

        for part in it {
            path.push(part.clone());
            let name = name.clone().with_parts(path.clone());

            if Some(&name) == current {
                html!(self, span {class => "name-part"} ~ part);
            } else {
                let url = self.type_url(&name)?;
                html!(self, a {class => "name-part", href => url} ~ part);
            }

            html!(self, span {class => "name-sep"} ~ "::");
        }

        let url = self.type_url(name)?;
        html!(self, a {class => "name-local", href => url} ~ local);
        Ok(())
    }

    /// Local name fully linked.
    fn full_name_without_package(&self, name: &RpName) -> Result<()> {
        let mut it = name.path.iter();
        let local = it
            .next_back()
            .ok_or_else(|| "local part of name required")?;

        let mut path = Vec::new();

        if let Some(ref prefix) = name.prefix {
            let package_url = self.package_url(&name.package);
            html!(self, a {class => "name-package", href => package_url} ~ prefix);
            html!(self, span {class => "name-sep"} ~ "::");
        }

        for part in it {
            path.push(part.clone());
            let name = name.clone().with_parts(path.clone());
            let url = self.type_url(&name)?;
            html!(self, a {class => "name-part", href => url} ~ part);
            html!(self, span {class => "name-sep"} ~ "::");
        }

        let url = self.type_url(name)?;
        html!(self, a {class => "name-local", href => url} ~ local);
        Ok(())
    }

    /// Write the name, but without a local part.
    fn name_until(&self, name: &RpName) -> Result<()> {
        for part in &name.path {
            html!(self, span {class => "name-part"} ~ part);
            html!(self, span {class => "name-sep"} ~ "::");
        }

        Ok(())
    }
}
