use pulldown_cmark as markdown;
use std::collections::HashMap;
use std::rc::Rc;
use super::*;

pub struct DocBackend {
    #[allow(dead_code)]
    options: DocOptions,
    pub env: Environment,
    package_prefix: Option<RpPackage>,
    pub theme: String,
    listeners: Box<DocListeners>,
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
    pub fn new(options: DocOptions,
               env: Environment,
               package_prefix: Option<RpPackage>,
               theme: String,
               listeners: Box<DocListeners>)
               -> DocBackend {
        DocBackend {
            options: options,
            env: env,
            package_prefix: package_prefix,
            theme: theme,
            listeners: listeners,
            themes: build_themes(),
        }
    }

    fn type_url(&self, pos: &RpPos, type_id: &RpTypeId) -> Result<String> {
        let (package, registered) = self.env
            .lookup(&type_id.package, &type_id.name)
            .map_err(|e| Error::pos(e.description().to_owned(), pos.clone()))?;

        if let Some(_) = type_id.name.prefix {
            let package = self.package(package);
            let package = self.package_file(&package);
            let fragment = registered.name().join("_");
            return Ok(format!("{}.html#{}", package, fragment));
        }

        let fragment = registered.name().join("_");
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

    fn write_markdown(&self, out: &mut DocBuilder, comment: &Vec<String>) -> Result<()> {
        if !comment.is_empty() {
            let comment = comment.join("\n");
            write!(out, "{}", Self::markdown(&comment))?;
        }

        Ok(())
    }

    fn write_description<'a, I>(&self, out: &mut DocBuilder, comment: I) -> Result<()>
        where I: IntoIterator<Item = &'a String>
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
        where I: IntoIterator<Item = &'b RpLoc<Rc<RpEnumVariant>>>
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
                        html!(out, td {class => "name"} ~ &variant.name);

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

    fn write_type(&self,
                  out: &mut DocBuilder,
                  pos: &RpPos,
                  type_id: &RpTypeId,
                  ty: &RpType)
                  -> Result<()> {
        write!(out, "<span class=\"ty\">")?;

        match *ty {
            RpType::Double => self.write_simple_type(out, "double")?,
            RpType::Float => self.write_simple_type(out, "float")?,
            RpType::Boolean => self.write_simple_type(out, "boolean")?,
            RpType::String => self.write_simple_type(out, "string")?,
            RpType::Bytes => self.write_simple_type(out, "bytes")?,
            RpType::Any => self.write_simple_type(out, "any")?,
            RpType::Signed { ref size } => {
                html!(out, span {class => "type-signed"} => {
                    html!(out, code {class => "type-name"} ~ "signed");

                    if let Some(ref size) = *size {
                        html!(out, span {class => "type-size-sep"} ~ "/");
                        html!(out, span {class => "type-size"} ~ format!("{}", size));
                    }
                });
            }
            RpType::Unsigned { ref size } => {
                html!(out, span {class => "type-unsigned"} => {
                    html!(out, code {class => "type-name"} ~ "unsigned");

                    if let Some(ref size) = *size {
                        html!(out, span {class => "type-size-sep"} ~ "/");
                        html!(out, span {class => "type-size"} ~ format!("{}", size));
                    }
                });
            }
            RpType::Name { ref name } => {
                let url = self.type_url(pos, &type_id.with_name(name.clone()))?;
                let name = name.parts.join(".");

                html!(out, span {class => "type-rp-name"} => {
                    html!(out, a {href => url} ~ name);
                });
            }
            RpType::Array { ref inner } => {
                html!(out, span {class => "type-array"} => {
                    html!(out, span {class => "type-array-left"} ~ "[");
                    self.write_type(out, pos, type_id, inner)?;
                    html!(out, span {class => "type-array-right"} ~ "]");
                });
            }
            RpType::Map { ref key, ref value } => {
                html!(out, span {class => "type-map"} => {
                    html!(out, span {class => "type-map-left"} ~ "{");
                    self.write_type(out, pos, type_id, key)?;
                    html!(out, span {class => "type-map-sep"} ~ "{");
                    self.write_type(out, pos, type_id, value)?;
                    html!(out, span {class => "type-map-right"} ~ "}");
                });
            }
        }

        write!(out, "</span>")?;
        Ok(())
    }

    fn write_fields<'b, I>(&self, out: &mut DocBuilder, type_id: &RpTypeId, fields: I) -> Result<()>
        where I: Iterator<Item = &'b RpLoc<RpField>>
    {
        html!(out, div {class => "fields"} => {
            html!(out, h2 {} ~ "Fields");

            html!(out, table {class => "spaced"} => {
                for field in fields {
                    let (field, pos) = field.ref_both();

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
                            self.write_type(out, pos, type_id, &field.ty)?;
                        });

                        html!(out, td {class => "description"} => {
                            self.write_markdown(out, &field.comment)?;
                        });
                    });
                }
            });
        });

        Ok(())
    }

    fn section_title(&self, out: &mut DocBuilder, ty: &str, name: &str, id: &str) -> Result<()> {
        html!(out, h1 {class => "section-title"} => {
            html!(out, a {class => "link", href => format!("#{}", id)} ~ name);
            html!(out, span {class => "type"} ~ ty);
        });

        Ok(())
    }

    pub fn write_doc<Body>(&self, out: &mut DocBuilder, body: Body) -> Result<()>
        where Body: FnOnce(&mut DocBuilder) -> Result<()>
    {
        html!(out, html {} => {
            html!(out, head {} => {
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

    fn write_endpoint_short(&self,
                            out: &mut DocBuilder,
                            index: usize,
                            body: &Rc<RpServiceBody>,
                            endpoint: &RpServiceEndpoint)
                            -> Result<()> {
        let method: String =
            endpoint.method.as_ref().map(AsRef::as_ref).unwrap_or("GET").to_owned();

        let id = self.endpoint_id(index, &method, body, endpoint);

        html!(out, div {class => format!("endpoint short {}", method.to_lowercase())} => {
            html!(out, a {class => "endpoint-title", href => format!("#{}", id)} => {
                html!(out, span {class => "method"} ~ method);
                html!(out, span {class => "url"} ~ endpoint.url);
            });

            html!(out, div {class => "endpoint-body"} => {
                self.write_description(out, endpoint.comment.iter().take(1))?;
            });
        });

        Ok(())
    }

    fn url_filter(&self, url: &str) -> String {
        url.replace(|c| match c {
                        'a'...'z' | 'A'...'Z' | '0'...'9' => false,
                        _ => true,
                    },
                    "_")
    }

    fn endpoint_id(&self,
                   index: usize,
                   method: &str,
                   body: &Rc<RpServiceBody>,
                   endpoint: &RpServiceEndpoint)
                   -> String {
        format!("{}_{}_{}_{}",
                method,
                self.url_filter(&body.name),
                self.url_filter(&endpoint.url),
                index)
    }

    fn write_endpoint(&self,
                      out: &mut DocBuilder,
                      index: usize,
                      type_id: &RpTypeId,
                      body: &Rc<RpServiceBody>,
                      endpoint: &RpServiceEndpoint)
                      -> Result<()> {
        let method: String =
            endpoint.method.as_ref().map(AsRef::as_ref).unwrap_or("GET").to_owned();

        let id = self.endpoint_id(index, &method, body, endpoint);

        html!(out, div {class => format!("endpoint {}", method.to_lowercase()), id => id} => {
            html!(out, h2 {class => "endpoint-title"} => {
                html!(out, span {class => "method"} ~ method);
                html!(out, a {class => "url", href => format!("#{}", id)} ~ endpoint.url);
            });

            html!(out, div {class => "endpoint-body"} => {
                self.write_description(out, &endpoint.comment)?;

                if !endpoint.accepts.is_empty() {
                    html!(out, h2 {} ~ "Accepts");

                    html!(out, table {class => "spaced"} => {
                        for accept in &endpoint.accepts {
                            html!(out, tr {} => {
                                let (ty, pos) = accept.ty.ref_both();

                                let accepts = accept.accepts
                                    .as_ref()
                                    .map(|m| format!("{}", m))
                                    .unwrap_or("*/*".to_owned());

                                html!(out, td {class => "mime"} => {
                                    html!(out, code {} ~ accepts)
                                });

                                html!(out, td {class => "type"} => {
                                    self.write_type(out, pos, type_id, ty)?;
                                });

                                html!(out, td {class => "description"} => {
                                    self.write_markdown(out, &accept.comment)?;
                                });
                            });
                        }
                    });
                }

                if !endpoint.returns.is_empty() {
                    html!(out, h2 {} ~ "Returns");

                    html!(out, table {class => "spaced"} => {
                        for response in &endpoint.returns {
                            html!(out, tr {} => {
                                let (ty, pos) = response.ty.ref_both();

                                let status = response.status
                                    .as_ref()
                                    .map(|status| format!("{}", status))
                                    .unwrap_or("<em>no status</em>".to_owned());

                                let produces = response.produces
                                    .as_ref()
                                    .map(|m| format!("{}", m))
                                    .unwrap_or("*/*".to_owned());

                                html!(out, td {class => "status"} ~ status);
                                html!(out, td {class => "mime"} => {
                                    html!(out, code {} ~ produces)
                                });

                                html!(out, td {class => "type"} => {
                                    self.write_type(out, pos, type_id, ty)?;
                                });

                                html!(out, td {class => "description"} => {
                                    self.write_markdown(out, &response.comment)?;
                                });
                            });
                        }
                    });
                }
            });
        });

        Ok(())
    }

    /// Write a packages index.
    ///
    /// * `current` if some value indicates which the current package is.
    pub fn write_packages(&self,
                          out: &mut DocBuilder,
                          packages: &Vec<RpVersionedPackage>,
                          current: Option<&RpVersionedPackage>)
                          -> Result<()> {
        html!(out, section {class => "section-content section-packages"} => {
            html!(out, h1 {class => "section-title"} ~ "Packages");

            html!(out, div {class => "section-body"} => {
                html!(out, ul {class => "packages-list"} => {
                    for package in packages {
                        let name = format!("{}", package);

                        if let Some(current) = current {
                            if package == current {
                                html!(out, li {} ~ format!("<b>{}</b>", name));
                                continue;
                            }
                        }

                        let package = self.package(package);
                        let url = format!("{}.{}", self.package_file(&package), EXT);

                        html!(out, li {} => {
                            html!(out, a {href => url} ~ name);
                        });
                    }
                });
            });
        });

        Ok(())
    }

    pub fn write_service_overview(&self,
                                  out: &mut DocBuilder,
                                  service_bodies: Vec<Rc<RpServiceBody>>)
                                  -> Result<()> {
        if service_bodies.is_empty() {
            return Ok(());
        }

        html!(out, section {class => "section-content section-service-overview"} => {
            html!(out, h1 {class => "section-title"} ~ "Services");

            html!(out, div {class => "section-body"} => {
                for body in service_bodies {
                    html!(out, h2 {} ~ body.name);

                    self.write_description(out, body.comment.iter().take(1))?;

                    for (index, endpoint) in body.endpoints.iter().enumerate() {
                        self.write_endpoint_short(out, index, &body, endpoint)?;
                    }
                }
            })
        });

        Ok(())
    }

    pub fn write_types_overview(&self, out: &mut DocBuilder, decls: Vec<RpDecl>) -> Result<()> {
        if decls.is_empty() {
            return Ok(());
        }

        html!(out, section {class => "section-content section-types-overview"} => {
            html!(out, h1 {class => "section-title"} ~ "Types");

            html!(out, div {class => "section-body"} => {
                for decl in decls {
                    let href = format!("#{}", decl.name());

                    html!(out, h2 {} => {
                        html!(out, a {href => href} ~ decl.name());
                    });

                    self.write_description(out, decl.comment().iter().take(1))?;
                }
            })
        });

        Ok(())
    }

    pub fn process_service(&self,
                           out: &mut DocCollector,
                           type_id: &RpTypeId,
                           _: &RpPos,
                           body: Rc<RpServiceBody>)
                           -> Result<()> {
        let mut new_service = out.new_service(body.clone());
        let mut out = DefaultDocBuilder::new(&mut new_service);

        html!(out, section {id => body.name, class => "section-content section-service"} => {
            self.section_title(&mut out, "service", &body.name, &body.name)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;

                for (index, endpoint) in body.endpoints.iter().enumerate() {
                    self.write_endpoint(&mut out, index, type_id, &body, endpoint)?;
                }
            });
        });

        Ok(())
    }

    pub fn process_enum(&self,
                        out: &mut DocCollector,
                        _: &RpTypeId,
                        _: &RpPos,
                        body: Rc<RpEnumBody>)
                        -> Result<()> {
        let mut new_enum = out.new_type(RpDecl::Enum(body.clone()));
        let mut out = DefaultDocBuilder::new(&mut new_enum);

        html!(out, section {id => body.name, class => "section-content section-enum"} => {
            self.section_title(&mut out, "enum", &body.name, &body.name)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;
                self.write_variants(&mut out, body.variants.iter())?;
            });
        });

        Ok(())
    }

    pub fn process_interface(&self,
                             out: &mut DocCollector,
                             type_id: &RpTypeId,
                             _: &RpPos,
                             body: Rc<RpInterfaceBody>)
                             -> Result<()> {
        let mut new_interface = out.new_type(RpDecl::Interface(body.clone()));
        let mut out = DefaultDocBuilder::new(&mut new_interface);

        html!(out, section {id => body.name, class => "section-content section-interface"} => {
            self.section_title(&mut out, "interface", &body.name, &body.name)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;

                if !body.sub_types.is_empty() {
                    html!(out, div {class => "sub-types"} => {
                        for (name, sub_type) in &body.sub_types {
                            let id = format!("{}_{}", body.name, sub_type.name);

                            html!(out, h2 {id => id, class => "sub-type-title"} => {
                                html!(out, a {class => "link", href => format!("#{}", id)} ~ name);
                            });

                            self.write_description(&mut out, &body.comment)?;

                            let fields = body.fields.iter().chain(sub_type.fields.iter());
                            self.write_fields(&mut out, type_id, fields)?;
                        }
                    });
                }
            });
        });

        Ok(())
    }

    pub fn process_type(&self,
                        out: &mut DocCollector,
                        type_id: &RpTypeId,
                        _: &RpPos,
                        body: Rc<RpTypeBody>)
                        -> Result<()> {
        let mut new_type = out.new_type(RpDecl::Type(body.clone()));
        let mut out = DefaultDocBuilder::new(&mut new_type);

        html!(out, section {id => body.name, class => "section-content section-type"} => {
            self.section_title(&mut out, "type", &body.name, &body.name)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;
                self.write_fields(&mut out, type_id, body.fields.iter())?;
            });
        });

        Ok(())
    }

    pub fn process_tuple(&self,
                         out: &mut DocCollector,
                         type_id: &RpTypeId,
                         _: &RpPos,
                         body: Rc<RpTupleBody>)
                         -> Result<()> {
        let mut new_tuple = out.new_type(RpDecl::Tuple(body.clone()));
        let mut out = DefaultDocBuilder::new(&mut new_tuple);

        html!(out, section {id => body.name, class => "section-content section-tuple"} => {
            self.section_title(&mut out, "tuple", &body.name, &body.name)?;

            html!(out, div {class => "section-body"} => {
                self.write_description(&mut out, &body.comment)?;
                self.write_fields(&mut out, type_id, body.fields.iter())?;
            });
        });

        Ok(())
    }
}

impl PackageUtils for DocBackend {
    fn version_package(input: &Version) -> String {
        format!("{}", input).replace(Self::package_version_unsafe, "_")
    }

    fn package_prefix(&self) -> &Option<RpPackage> {
        &self.package_prefix
    }
}

impl Backend for DocBackend {
    fn compiler<'a>(&'a self, options: CompilerOptions) -> Result<Box<Compiler<'a> + 'a>> {
        Ok(Box::new(DocCompiler {
            out_path: options.out_path,
            processor: self,
        }))
    }

    fn verify(&self) -> Result<Vec<Error>> {
        Ok(vec![])
    }
}
