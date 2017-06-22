//! # Collector of results from the doc backend

use std::fmt;
use std::fmt::Write;
use std::rc::Rc;
use super::*;

pub struct DocWriter<'a> {
    dest: &'a mut Vec<String>,
    buffer: String,
}

impl<'a> DocBuilder for DocWriter<'a> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        Write::write_str(&mut self.buffer, string)
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        Write::write_fmt(&mut self.buffer, args)
    }
}

impl<'a> Drop for DocWriter<'a> {
    fn drop(&mut self) {
        self.dest.push(self.buffer.clone());
    }
}

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

        DocWriter {
            dest: &mut self.services,
            buffer: String::new(),
        }
    }

    pub fn new_service_overview(&mut self) -> DocWriter {
        DocWriter {
            dest: &mut self.service_overviews,
            buffer: String::new(),
        }
    }

    pub fn new_types_overview(&mut self) -> DocWriter {
        DocWriter {
            dest: &mut self.types_overview,
            buffer: String::new(),
        }
    }

    pub fn new_package(&mut self) -> DocWriter {
        DocWriter {
            dest: &mut self.packages,
            buffer: String::new(),
        }
    }

    pub fn new_type(&mut self, decl: RpDecl) -> DocWriter {
        self.decl_bodies.push(decl);

        DocWriter {
            dest: &mut self.types,
            buffer: String::new(),
        }
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
        let mut out = String::new();

        compiler.processor
            .write_doc(&mut out, move |out| {
                if let Some(package_title) = self.package_title {
                    html!(out, h1 {class => "document-title"} => {
                        write!(out, "Package: {}", package_title)?;
                    });
                }

                html!(out, div {class => "grid-container"} => {
                    html!(out, div {class => "grid-packages"} => {
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

        Ok(out.into_bytes())
    }
}
