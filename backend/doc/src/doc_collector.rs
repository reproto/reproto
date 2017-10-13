//! # Collector of results from the doc backend

use backend::IntoBytes;
use backend::errors::*;
use core::{RpEnumBody, RpInterfaceBody, RpServiceBody, RpTupleBody, RpTypeBody};
use doc_builder::DefaultDocBuilder;
use doc_compiler::DocCompiler;
use doc_writer::DocWriter;
use macros::FormatAttribute;

#[derive(Debug, Clone)]
pub enum DocDecl<'p> {
    Enum(&'p RpEnumBody),
    Interface(&'p RpInterfaceBody),
    Type(&'p RpTypeBody),
    Tuple(&'p RpTupleBody),
}

impl<'p> DocDecl<'p> {
    pub fn local_name(&self) -> &str {
        use self::DocDecl::*;

        match *self {
            Type(ref body) => body.local_name.as_str(),
            Interface(ref body) => body.local_name.as_str(),
            Enum(ref body) => body.local_name.as_str(),
            Tuple(ref body) => body.local_name.as_str(),
        }
    }

    pub fn comment(&self) -> &[String] {
        use self::DocDecl::*;

        match *self {
            Type(ref body) => &body.comment,
            Interface(ref body) => &body.comment,
            Enum(ref body) => &body.comment,
            Tuple(ref body) => &body.comment,
        }
    }
}

pub struct DocCollector<'p> {
    package_title: Option<String>,
    packages: Vec<String>,
    services: Vec<String>,
    service_overviews: Vec<String>,
    types_overview: Vec<String>,
    types: Vec<String>,
    pub service_bodies: Vec<&'p RpServiceBody>,
    pub decl_bodies: Vec<DocDecl<'p>>,
}

impl<'p> DocCollector<'p> {
    pub fn set_package_title(&mut self, title: String) {
        self.package_title = Some(title);
    }

    pub fn new_service(&mut self, service_body: &'p RpServiceBody) -> DocWriter {
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

    pub fn new_type(&mut self, decl: DocDecl<'p>) -> DocWriter {
        self.decl_bodies.push(decl);
        DocWriter::new(&mut self.types)
    }
}

impl<'p> Default for DocCollector<'p> {
    fn default() -> DocCollector<'p> {
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
}

impl<'p> IntoBytes<DocCompiler<'p>> for DocCollector<'p> {
    fn into_bytes(self, compiler: &DocCompiler<'p>) -> Result<Vec<u8>> {
        let mut buffer = String::new();

        compiler.backend.write_doc(
            &mut DefaultDocBuilder::new(&mut buffer),
            move |out| {
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
            },
        )?;

        Ok(buffer.into_bytes())
    }
}
