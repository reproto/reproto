//! # Collector of results from the doc backend

use macros::FormatAttribute;
use std::rc::Rc;
use super::*;

pub struct DocCollector {
    package_title: Option<String>,
    packages: Vec<String>,
    services: Vec<String>,
    service_overviews: Vec<String>,
    types_overview: Vec<String>,
    types: Vec<String>,
    pub service_bodies: Vec<Rc<RpServiceBody>>,
    pub decl_bodies: Vec<RpDecl>,
}

impl DocCollector {
    pub fn set_package_title(&mut self, title: String) {
        self.package_title = Some(title);
    }

    pub fn new_service(&mut self, service_body: Rc<RpServiceBody>) -> DocWriter {
        self.service_bodies.push(service_body);
        DocWriter::new(&mut self.services)
    }

    pub fn new_service_overview(&mut self) -> DocWriter {
        DocWriter::new(&mut self.service_overviews)
    }

    pub fn new_types_overview(&mut self) -> DocWriter {
        DocWriter::new(&mut self.types_overview)
    }

    pub fn new_package(&mut self) -> DocWriter {
        DocWriter::new(&mut self.packages)
    }

    pub fn new_type(&mut self, decl: RpDecl) -> DocWriter {
        self.decl_bodies.push(decl);
        DocWriter::new(&mut self.types)
    }
}

impl<'a> Collecting<'a> for DocCollector {
    type Processor = DocCompiler<'a>;

    fn new() -> Self {
        DocCollector {
            package_title: None,
            packages: Vec::new(),
            services: Vec::new(),
            service_overviews: Vec::new(),
            types_overview: Vec::new(),
            types: Vec::new(),
            service_bodies: Vec::new(),
            decl_bodies: Vec::new(),
        }
    }

    fn into_bytes(self, compiler: &Self::Processor) -> Result<Vec<u8>> {
        let mut buffer = String::new();

        compiler.backend
            .write_doc(&mut DefaultDocBuilder::new(&mut buffer), move |out| {
                if let Some(package_title) = self.package_title {
                    html!(out, h1 {class => "document-title"} => {
                        write!(out, "Package: {}", package_title)?;
                    });
                }

                html!(out, div {class => "grid-container"} => {
                    html!(out, div {class => "grid-overview"} => {
                        if let Some(package) = self.packages.iter().nth(0) {
                            out.write_str(package.as_str())?;
                        }

                        if let Some(service_overview) = self.service_overviews.iter().nth(0) {
                            out.write_str(service_overview.as_str())?;
                        }

                        if let Some(decl_overview) = self.types_overview.iter().nth(0) {
                            out.write_str(decl_overview.as_str())?;
                        }
                    });

                    html!(out, div {class => "grid-decls"} => {
                        for service in self.services {
                            out.write_str(service.as_str())?;
                        }

                        for ty in self.types {
                            out.write_str(ty.as_str())?;
                        }
                    });
                });

                Ok(())
            })?;

        Ok(buffer.into_bytes())
    }
}
