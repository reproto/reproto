use pulldown_cmark as markdown;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
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

    fn write_markdown(&self, out: &mut FmtWrite, comment: &Vec<String>) -> Result<()> {
        if !comment.is_empty() {
            let comment = comment.join("\n");
            write!(out, "{}", Self::markdown(&comment))?;
        }

        Ok(())
    }

    fn write_description(&self, out: &mut FmtWrite, comment: &Vec<String>) -> Result<()> {
        if comment.is_empty() {
            write!(out,
                   "<div class=\"description\"><em>no description</em></div>")?;
        } else {
            let comment = comment.join("\n");

            write!(out,
                   "<div class=\"description\">{}</div>",
                   Self::markdown(&comment))?;
        }

        Ok(())
    }

    fn write_variants<'b, I>(&self, out: &mut FmtWrite, variants: I) -> Result<()>
        where I: Iterator<Item = &'b RpLoc<Rc<RpEnumVariant>>>
    {
        write!(out, "<div class=\"variants\">")?;

        for variant in variants {
            write!(out, "<div class=\"variant\">")?;
            write!(out, "<h4 class=\"name\">{}</h4>", variant.name)?;

            self.write_description(out, &variant.comment)?;

            write!(out, "</div>")?;
        }

        write!(out, "</div>")?;

        Ok(())
    }

    fn write_type(&self,
                  out: &mut FmtWrite,
                  pos: &RpPos,
                  type_id: &RpTypeId,
                  ty: &RpType)
                  -> Result<()> {
        write!(out, "<span class=\"ty\">")?;

        match *ty {
            RpType::Double => {
                write!(out, "<span class=\"ty-double\">double</span>")?;
            }
            RpType::Float => {
                write!(out, "<span class=\"ty-float\">float</span>")?;
            }
            RpType::Signed { ref size } => {
                if let Some(ref size) = *size {
                    write!(out, "<span class=\"ty-signed\">signed/{}</span>", size)?;
                } else {
                    write!(out, "<span class=\"ty-signed\">signed</span>")?;
                }
            }
            RpType::Unsigned { ref size } => {
                if let Some(ref size) = *size {
                    write!(out, "<span class=\"ty-unsigned\">unsigned/{}</span>", size)?;
                } else {
                    write!(out, "<span class=\"ty-unsigned\">unsigned</span>")?;
                }
            }
            RpType::Boolean => {
                write!(out, "<span class=\"ty-boolean\">boolean</span>")?;
            }
            RpType::String => {
                write!(out, "<span class=\"ty-string\">string</span>")?;
            }
            RpType::Bytes => {
                write!(out, "<span class=\"ty-bytes\">bytes</span>")?;
            }
            RpType::Any => {
                write!(out, "<span class=\"ty-any\">any</span>")?;
            }
            RpType::Name { ref name } => {
                let url = self.type_url(pos, &type_id.with_name(name.clone()))?;
                let name = name.parts.join(".");

                write!(out, "<span class=\"ty-name\">")?;
                write!(out, "<a href=\"{url}\">{name}</a>", url = url, name = name)?;
                write!(out, "</span>")?;
            }
            RpType::Array { ref inner } => {
                write!(out, "<span class=\"ty-array\">")?;
                write!(out, "<span class=\"ty-array-left\">[</span>")?;
                self.write_type(out, pos, type_id, inner)?;
                write!(out, "<span class=\"ty-array-right\">]</span>")?;
                write!(out, "</span>")?;
            }
            RpType::Map { ref key, ref value } => {
                write!(out, "<span class=\"ty-map\">")?;
                write!(out, "<span class=\"ty-map-key\">{{</span>")?;
                self.write_type(out, pos, type_id, key)?;
                write!(out, "<span class=\"ty-map-separator\">:</span>")?;
                self.write_type(out, pos, type_id, value)?;
                write!(out, "<span class=\"ty-map-value\">}}</span>")?;
                write!(out, "</span>")?;
            }
        }

        write!(out, "</span>")?;
        Ok(())
    }

    fn write_fields<'b, I>(&self, out: &mut FmtWrite, type_id: &RpTypeId, fields: I) -> Result<()>
        where I: Iterator<Item = &'b RpLoc<RpField>>
    {
        write!(out, "<div class=\"fields\">")?;

        for field in fields {
            let (field, pos) = field.ref_both();

            write!(out, "<div class=\"field\">")?;

            let mut name = format!("<span>{}</span>", field.ident());
            let mut class = "name".to_owned();

            if field.is_optional() {
                class = format!("{} optional", class);
                name = format!("{}<span class=\"modifier\">?:</span>", name);
            } else {
                name = format!("{}<span class=\"modifier\">:</span>", name);
            };

            write!(out, "<div class=\"{class}\">", class = class)?;
            write!(out, "{name}", name = name)?;
            self.write_type(out, pos, type_id, &field.ty)?;
            write!(out, "</div>")?;

            self.write_description(out, &field.comment)?;

            write!(out, "</div>")?;
        }

        write!(out, "</div>")?;

        Ok(())
    }

    fn section_title(&self, out: &mut FmtWrite, ty: &str, name: &str) -> Result<()> {
        write!(out, "<h1>")?;
        write!(out, "{name}", name = name)?;
        write!(out, "<span class=\"type\">{}</span>", ty)?;
        write!(out, "</h1>")?;

        Ok(())
    }

    pub fn write_doc<Body>(&self, out: &mut FmtWrite, body: Body) -> Result<()>
        where Body: FnOnce(&mut FmtWrite) -> Result<()>
    {
        write!(out, "<html>")?;
        write!(out, "<head>")?;

        write!(out,
               "<link rel=\"stylesheet\" type=\"text/css\" href=\"{normalize_css}\">",
               normalize_css = NORMALIZE_CSS_NAME)?;

        write!(out,
               "<link rel=\"stylesheet\" type=\"text/css\" href=\"{doc_css}\">",
               doc_css = DOC_CSS_NAME)?;

        write!(out, "</head>")?;
        write!(out, "<body>")?;

        body(out)?;

        write!(out, "</body>")?;
        write!(out, "</html>")?;

        Ok(())
    }

    pub fn process_service(&self,
                           out: &mut DocCollector,
                           type_id: &RpTypeId,
                           _: &RpPos,
                           body: Rc<RpServiceBody>)
                           -> Result<()> {
        write!(out,
               "<section id=\"{}\" class=\"section-service\">",
               body.name)?;

        self.section_title(out, "service", &body.name)?;
        self.write_description(out, &body.comment)?;

        for endpoint in &body.endpoints {
            let method: String =
                endpoint.method.as_ref().map(AsRef::as_ref).unwrap_or("GET").to_owned();

            write!(out,
                   "<h2 class=\"endpoint-title {method}\">",
                   method = method.to_lowercase())?;

            write!(out, "<span class=\"method\">{}</span>", method)?;
            write!(out, "<span class=\"url\">{}</span>", endpoint.url)?;
            write!(out, "</h2>")?;

            write!(out, "<div class=\"endpoint-body\">")?;

            self.write_description(out, &endpoint.comment)?;

            if !endpoint.accepts.is_empty() {
                write!(out, "<h4>Accepts:</h4>")?;

                for accept in &endpoint.accepts {
                    write!(out, "<div class=\"accept\">")?;
                    write!(out, "<span>{}</span>", accept)?;
                    write!(out, "</div>")?;
                }
            }

            if !endpoint.returns.is_empty() {
                write!(out, "<table class=\"returns\">")?;

                for response in &endpoint.returns {
                    write!(out, "<tr>")?;

                    let (ty, pos) = response.ty.ref_both();

                    let status = response.status
                        .as_ref()
                        .map(|status| format!("{}", status))
                        .unwrap_or("<em>no status</em>".to_owned());

                    let produces = response.produces
                        .as_ref()
                        .map(|m| format!("{}", m))
                        .unwrap_or("*/*".to_owned());

                    write!(out, "<td class=\"status\">{}</td>", status)?;
                    write!(out, "<td class=\"content-type\">{}</td>", produces)?;

                    write!(out, "<td class=\"ty\">")?;
                    self.write_type(out, pos, type_id, ty)?;
                    write!(out, "</td>")?;

                    write!(out, "<td class=\"description\">")?;
                    self.write_markdown(out, &response.comment)?;
                    write!(out, "</td>")?;

                    write!(out, "</tr>")?;
                }

                write!(out, "</table>")?;
            }

            write!(out, "</div>")?;
        }

        write!(out, "</section>")?;
        Ok(())
    }

    pub fn process_enum(&self,
                        out: &mut DocCollector,
                        _: &RpTypeId,
                        _: &RpPos,
                        body: Rc<RpEnumBody>)
                        -> Result<()> {
        write!(out, "<section id=\"{}\" class=\"section-enum\">", body.name)?;

        self.section_title(out, "enum", &body.name)?;
        self.write_description(out, &body.comment)?;
        self.write_variants(out, body.variants.iter())?;

        write!(out, "</section>")?;
        Ok(())
    }

    pub fn process_interface(&self,
                             out: &mut DocCollector,
                             type_id: &RpTypeId,
                             _: &RpPos,
                             body: Rc<RpInterfaceBody>)
                             -> Result<()> {
        write!(out,
               "<section id=\"{}\" class=\"section-interface\">",
               body.name)?;

        self.section_title(out, "interface", &body.name)?;
        self.write_description(out, &body.comment)?;

        for (name, sub_type) in &body.sub_types {
            let id = format!("{}_{}", body.name, sub_type.name);
            write!(out, "<h2 id=\"{id}\">{name}</h2>", id = id, name = name)?;

            let fields = body.fields.iter().chain(sub_type.fields.iter());

            self.write_description(out, &sub_type.comment)?;
            self.write_fields(out, type_id, fields)?;
        }

        write!(out, "</section>")?;
        Ok(())
    }

    pub fn process_type(&self,
                        out: &mut DocCollector,
                        type_id: &RpTypeId,
                        _: &RpPos,
                        body: Rc<RpTypeBody>)
                        -> Result<()> {
        write!(out, "<section id=\"{}\" class=\"section-type\">", body.name)?;

        self.section_title(out, "type", &body.name)?;
        self.write_description(out, &body.comment)?;
        self.write_fields(out, type_id, body.fields.iter())?;

        write!(out, "</section>")?;
        Ok(())
    }

    pub fn process_tuple(&self,
                         out: &mut DocCollector,
                         type_id: &RpTypeId,
                         _: &RpPos,
                         body: Rc<RpTupleBody>)
                         -> Result<()> {
        write!(out,
               "<section id=\"{}\" class=\"section-tuple\">",
               body.name)?;

        self.section_title(out, "tuple", &body.name)?;
        self.write_description(out, &body.comment)?;
        self.write_fields(out, type_id, body.fields.iter())?;

        write!(out, "</section>")?;
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
