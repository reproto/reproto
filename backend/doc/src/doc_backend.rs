//! Backend module for Documentation.

use super::{DOC_CSS_NAME, EXT, NORMALIZE_CSS_NAME};
use backend::{Environment, PackageUtils};
use backend::errors::*;
use core::{ForEachLoc, Loc, RpEndpoint, RpEnumBody, RpEnumVariant, RpField, RpInterfaceBody,
           RpName, RpPackage, RpServiceBody, RpTupleBody, RpType, RpTypeBody, RpVersionedPackage,
           Version, WithPos};
use doc_builder::{DefaultDocBuilder, DocBuilder};
use doc_collector::{DocCollector, DocDecl};
use doc_listeners::DocListeners;
use doc_options::DocOptions;
use escape::Escape;
use macros::FormatAttribute;
use pulldown_cmark as markdown;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DocBackend {
    pub env: Environment,
    #[allow(dead_code)]
    options: DocOptions,
    listeners: Box<DocListeners>,
    pub theme: String,
    pub themes: HashMap<&'static str, &'static [u8]>,
}

include!(concat!(env!("OUT_DIR"), "/themes.rs"));

fn build_themes() -> HashMap<&'static str, &'static [u8]> {
    let mut m = HashMap::new();

    for (key, value) in build_themes_vec() {
        m.insert(key, value);
    }

    m
}

impl DocBackend {
    pub fn new(
        env: Environment,
        options: DocOptions,
        listeners: Box<DocListeners>,
        theme: String,
    ) -> DocBackend {
        DocBackend {
            env: env,
            options: options,
            listeners: listeners,
            theme: theme,
            themes: build_themes(),
        }
    }

    pub fn verify(&self) -> Result<()> {
        Ok(())
    }

    fn type_url(&self, name: &RpName) -> Result<String> {
        let registered = self.env.lookup(name)?;

        let fragment = registered.local_name(name, |p| p.join("_"), |c| c.join("_"));

        if let Some(_) = name.prefix {
            let package = self.package(&name.package);
            let package = self.package_file(&package);
            return Ok(format!("{}.html#{}", package, fragment));
        }

        return Ok(format!("#{}", fragment));
    }

    fn markdown(input: &str) -> String {
        let p = markdown::Parser::new(input);
        let mut s = String::new();
        markdown::html::push_html(&mut s, p);
        s
    }

    pub fn package_file(&self, package: &RpPackage) -> String {
        package.parts.join("_")
    }

    fn write_markdown(&self, out: &mut DocBuilder, comment: &[String]) -> Result<()> {
        if !comment.is_empty() {
            let comment = comment.join("\n");
            write!(out, "{}", Self::markdown(&comment))?;
        }

        Ok(())
    }

    fn write_description<'a, I>(&self, out: &mut DocBuilder, comment: I) -> Result<()>
    where
        I: IntoIterator<Item = &'a String>,
    {
        let mut it = comment.into_iter().peekable();

        if it.peek().is_some() {
            let comment = it.map(ToOwned::to_owned).collect::<Vec<_>>();
            let comment = comment.join("\n");
            html!(out, div { class => "description" } ~ Self::markdown(&comment));
        }

        Ok(())
    }

    fn write_variants<'b, I>(&self, out: &mut DocBuilder, variants: I) -> Result<()>
    where
        I: IntoIterator<Item = &'b Rc<Loc<RpEnumVariant>>>,
    {
        let mut it = variants.into_iter().peekable();

        if it.peek().is_none() {
            return Ok(());
        }

        html!(out, div {class => "variants"} => {
            html!(out, h2 {} ~ "Variants");

            html!(out, table {class => "spaced"} => {
                for variant in it {
                    html!(out, tr {} => {
                        html!(out, td {class => "name"} ~ variant.local_name.as_ref());

                        html!(out, td {class => "description"} => {
                            self.write_description(out, &variant.comment)?;
                        });
                    });
                }
            });
        });

        Ok(())
    }

    fn write_simple_type(&self, out: &mut DocBuilder, name: &'static str) -> Result<()> {
        html!(out, span {class => format!("type-{}", name)} => {
            html!(out, code {class => "type-name"} ~ name);
        });

        Ok(())
    }

    fn write_type(&self, out: &mut DocBuilder, ty: &RpType) -> Result<()> {
        use self::RpType::*;

        write!(out, "<span class=\"ty\">")?;

        match *ty {
            Double => self.write_simple_type(out, "double")?,
            Float => self.write_simple_type(out, "float")?,
            Boolean => self.write_simple_type(out, "boolean")?,
            String => self.write_simple_type(out, "string")?,
            DateTime => self.write_simple_type(out, "datetime")?,
            Bytes => self.write_simple_type(out, "bytes")?,
            Any => self.write_simple_type(out, "any")?,
            Signed { ref size } => {
                html!(out, span {class => "type-signed"} => {
                    html!(out, code {class => "type-name"} ~ "signed");

                    if let Some(ref size) = *size {
                        html!(out, span {class => "type-size-sep"} ~ "/");
                        html!(out, span {class => "type-size"} ~ format!("{}", size));
                    }
                });
            }
            Unsigned { ref size } => {
                html!(out, span {class => "type-unsigned"} => {
                    html!(out, code {class => "type-name"} ~ "unsigned");

                    if let Some(ref size) = *size {
                        html!(out, span {class => "type-size-sep"} ~ "/");
                        html!(out, span {class => "type-size"} ~ format!("{}", size));
                    }
                });
            }
            Name { ref name } => {
                let url = self.type_url(name)?;
                let name = name.join("::");

                html!(out, span {class => "type-rp-name"} => {
                    html!(out, a {href => url} ~ name);
                });
            }
            Array { ref inner } => {
                html!(out, span {class => "type-array"} => {
                    html!(out, span {class => "type-array-left"} ~ "[");
                    self.write_type(out, inner)?;
                    html!(out, span {class => "type-array-right"} ~ "]");
                });
            }
            Map { ref key, ref value } => {
                html!(out, span {class => "type-map"} => {
                    html!(out, span {class => "type-map-left"} ~ "{");
                    self.write_type(out, key)?;
                    html!(out, span {class => "type-map-sep"} ~ ":");
                    self.write_type(out, value)?;
                    html!(out, span {class => "type-map-right"} ~ "}");
                });
            }
        }

        write!(out, "</span>")?;
        Ok(())
    }

    fn write_field(&self, out: &mut DocBuilder, field: &RpField) -> Result<()> {
        let mut classes = vec!["field"];

        if field.is_optional() {
            classes.push("optional");
        } else {
            classes.push("required");
        }

        html!(out, tr {classes => classes} => {
            html!(out, td {class => "mime"} => {
                let ident = field.ident();
                let name = field.name();

                html!(out, span {class => "field-ident"} ~ ident);

                if field.is_optional() {
                    html!(out, span {class => "field-modifier"} ~ "?");
                }

                if name != ident {
                    html!(out, span {class => "field-alias"} => {
                        html!(out, span {class => "field-alias-as"} ~ "as");
                        html!(out, code {class => "field-alias-name"} ~ format!("\"{}\"", name));
                    });
                }
            });

            html!(out, td {class => "type"} => {
                self.write_type(out, &field.ty)?;
            });

            html!(out, td {class => "description"} => {
                self.write_markdown(out, &field.comment)?;
            });
        });

        Ok(())
    }

    fn write_fields<'b, I>(&self, out: &mut DocBuilder, fields: I) -> Result<()>
    where
        I: Iterator<Item = &'b Loc<RpField>>,
    {
        html!(out, div {class => "fields"} => {
            html!(out, h2 {} ~ "Fields");

            html!(out, table {class => "spaced"} => {
                fields.for_each_loc(|field| {
                    self.write_field(out, field)
                })?;
            });
        });

        Ok(())
    }

    fn section_title(&self, out: &mut DocBuilder, ty: &str, name: &str, id: &str) -> Result<()> {
        html!(out, h1 {class => "section-title"} => {
            html!(out, a {class => "link", href => format!("#{}", id)} ~ Escape(name));
            html!(out, span {class => "type"} ~ ty);
        });

        Ok(())
    }

    pub fn write_doc<Body>(&self, out: &mut DocBuilder, body: Body) -> Result<()>
    where
        Body: FnOnce(&mut DocBuilder) -> Result<()>,
    {
        html!(out, html {} => {
            html!(out, head {} => {
                html!(@open out, meta {charset => "utf-8"});
                out.new_line()?;

                html!(@open out, meta {
                    name => "viewport",
                    content => "width=device-width, initial-scale=1.0"
                });
                out.new_line()?;

                html!(@open out, link {
                    rel => "stylesheet", type => "text/css", href => NORMALIZE_CSS_NAME
                });
                out.new_line()?;

                html!(@open out, link {
                    rel => "stylesheet", type => "text/css", href => DOC_CSS_NAME
                });
            });

            html!(out, body {} => {
                body(out)?;
            });
        });

        Ok(())
    }

    /// Write the name of an endpoint.
    fn write_endpoint_name(&self, out: &mut DocBuilder, endpoint: &RpEndpoint) -> Result<()> {
        html!(out, span {class => "id"} ~ Escape(endpoint.id.as_str()));
        html!(out, span {class => "name"} ~ Escape(endpoint.name.as_str()));
        Ok(())
    }

    /// Write a short section linking to and describing an endpoint.
    fn write_endpoint_short(
        &self,
        out: &mut DocBuilder,
        body: &RpServiceBody,
        endpoint: &RpEndpoint,
    ) -> Result<()> {
        let id = format!("{}_{}", body.name, endpoint.id_parts(Self::fragment_filter).join("_"));

        html!(out, div {class => "endpoint short"} => {
            html!(out, a {class => "endpoint-title", href => format!("#{}", id)} => {
                self.write_endpoint_name(out, endpoint)?;
            });

            if !endpoint.comment.is_empty() {
                html!(out, div {class => "endpoint-body"} => {
                    self.write_description(out, endpoint.comment.iter().take(1))?;
                });
            }
        });

        Ok(())
    }

    fn fragment_filter(url: &str) -> String {
        let mut bytes = [0u8; 4];
        let mut buffer = String::with_capacity(url.len());

        for c in url.chars() {
            let encode = match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' => false,
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

    fn write_endpoint(
        &self,
        out: &mut DocBuilder,
        body: &RpServiceBody,
        endpoint: &RpEndpoint,
    ) -> Result<()> {
        let id = format!("{}_{}", body.name, endpoint.id_parts(Self::fragment_filter).join("_"));

        html!(out, div {class => "endpoint", id => id} => {
            html!(out, h2 {class => "endpoint-title"} => {
                self.write_endpoint_name(out, endpoint)?;
            });

            html!(out, div {class => "endpoint-body"} => {
                self.write_description(out, &endpoint.comment)?;

                if let Some(request) = endpoint.request.as_ref().take().as_ref() {
                    html!(out, h2 {} ~ "Request");

                    html!(out, div {class => "type"} => {
                        let (req, pos) = request.as_ref_pair();
                        self.write_type(out, req.ty()).with_pos(pos)?;
                    });
                }

                if let Some(response) = endpoint.response.as_ref().take().as_ref() {
                    html!(out, h2 {} ~ "Response");

                    html!(out, div {class => "type"} => {
                        let (res, pos) = response.as_ref_pair();
                        self.write_type(out, res.ty()).with_pos(pos)?;
                    });
                }
            });
        });

        Ok(())
    }

    /// Write a packages index.
    ///
    /// * `current` if some value indicates which the current package is.
    pub fn write_packages(
        &self,
        out: &mut DocBuilder,
        packages: &[RpVersionedPackage],
        current: Option<&RpVersionedPackage>,
    ) -> Result<()> {
        html!(out, section {class => "section-content section-packages"} => {
            html!(out, h1 {class => "section-title"} ~ "Packages");

            html!(out, div {class => "section-body"} => {
                html!(out, ul {class => "packages-list"} => {
                    for package in packages {
                        let name = format!("{}", package);

                        if let Some(current) = current {
                            if package == current {
                                html!(out, li {} ~ format!("<b>{}</b>", Escape(name.as_ref())));
                                continue;
                            }
                        }

                        let package = self.package(package);
                        let url = format!("{}.{}", self.package_file(&package), EXT);

                        html!(out, li {} => {
                            html!(out, a {href => url} ~ Escape(name.as_ref()));
                        });
                    }
                });
            });
        });

        Ok(())
    }

    pub fn write_service_overview(
        &self,
        out: &mut DocBuilder,
        service_bodies: Vec<&RpServiceBody>,
    ) -> Result<()> {
        if service_bodies.is_empty() {
            return Ok(());
        }

        html!(out, section {class => "section-content section-service-overview"} => {
            html!(out, h1 {class => "section-title"} ~ "Services");

            html!(out, div {class => "section-body"} => {
                for body in service_bodies {
                    html!(out, h2 {} ~ &body.name);

                    self.write_description(out, body.comment.iter().take(1))?;

                    for endpoint in body.endpoints.values() {
                        self.write_endpoint_short(out, &body, endpoint)?;
                    }
                }
            })
        });

        Ok(())
    }

    pub fn write_types_overview(&self, out: &mut DocBuilder, decls: Vec<DocDecl>) -> Result<()> {
        if decls.is_empty() {
            return Ok(());
        }

        html!(out, section {class => "section-content section-types-overview"} => {
            html!(out, h1 {class => "section-title"} ~ "Types");

            html!(out, div {class => "section-body"} => {
                for decl in decls {
                    let href = format!("#{}", decl.local_name());

                    html!(out, h2 {} => {
                        html!(out, a {href => href} ~ decl.local_name());
                    });

                    self.write_description(out, decl.comment().iter().take(1))?;
                }
            })
        });

        Ok(())
    }

    pub fn process_service<'p>(
        &self,
        out: &mut DocCollector<'p>,
        body: &'p RpServiceBody,
    ) -> Result<()> {
        let mut new_service = out.new_service(body);
        let mut out = DefaultDocBuilder::new(&mut new_service);

        let title_text = body.name.join("::");
        let id = body.name.join("_");

        html!(out, section {id => &id, class => "section-content section-service"} => {
            self.section_title(&mut out, "service", &title_text, &id)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;

                for endpoint in body.endpoints.values() {
                    self.write_endpoint(&mut out, body, endpoint)?;
                }
            });
        });

        Ok(())
    }

    pub fn process_enum<'p>(&self, out: &mut DocCollector<'p>, body: &'p RpEnumBody) -> Result<()> {
        let mut new_enum = out.new_type(DocDecl::Enum(body));
        let mut out = DefaultDocBuilder::new(&mut new_enum);

        let title_text = body.name.join("::");
        let id = body.name.join("_");

        html!(out, section {id => &id, class => "section-content section-enum"} => {
            self.section_title(&mut out, "enum", &title_text, &id)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;
                self.write_variants(&mut out, body.variants.iter())?;
            });
        });

        Ok(())
    }

    pub fn process_interface<'p>(
        &self,
        out: &mut DocCollector<'p>,
        body: &'p RpInterfaceBody,
    ) -> Result<()> {
        let mut new_interface = out.new_type(DocDecl::Interface(body));
        let mut out = DefaultDocBuilder::new(&mut new_interface);

        let title_text = body.name.join("::");
        let id = body.name.join("_");

        html!(out, section {id => &id, class => "section-content section-interface"} => {
            self.section_title(&mut out, "interface", &title_text, &id)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;

                if !body.sub_types.is_empty() {
                    html!(out, div {class => "sub-types"} => {
                        for sub_type in body.sub_types.values() {
                            let id = format!("{}_{}", body.name, sub_type.name);

                            html!(out, h2 {id => id, class => "sub-type-title"} => {
                                html!(out, a {class => "link", href => format!("#{}", id)} ~
                                      sub_type.local_name);
                            });

                            self.write_description(&mut out, &body.comment)?;

                            let fields = body.fields.iter().chain(sub_type.fields.iter());
                            self.write_fields(&mut out, fields)?;
                        }
                    });
                }
            });
        });

        Ok(())
    }

    pub fn process_type<'p>(&self, out: &mut DocCollector<'p>, body: &'p RpTypeBody) -> Result<()> {
        let mut new_type = out.new_type(DocDecl::Type(body));
        let mut out = DefaultDocBuilder::new(&mut new_type);

        let title_text = body.name.join("::");
        let id = body.name.join("_");

        html!(out, section {id => &id, class => "section-content section-type"} => {
            self.section_title(&mut out, "type", &title_text, &id)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;
                self.write_fields(&mut out, body.fields.iter())?;
            });
        });

        Ok(())
    }

    pub fn process_tuple<'p>(
        &self,
        out: &mut DocCollector<'p>,
        body: &'p RpTupleBody,
    ) -> Result<()> {
        let mut new_tuple = out.new_type(DocDecl::Tuple(body));
        let mut out = DefaultDocBuilder::new(&mut new_tuple);

        let id = body.name.join("_");
        let title_text = body.name.join("::");

        html!(out, section {id => &id, class => "section-content section-tuple"} => {
            self.section_title(&mut out, "tuple", &title_text, &id)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;
                self.write_fields(&mut out, body.fields.iter())?;
            });
        });

        Ok(())
    }
}

impl PackageUtils for DocBackend {
    fn version_package(input: &Version) -> String {
        format!("{}", input).replace(Self::package_version_unsafe, "_")
    }
}
