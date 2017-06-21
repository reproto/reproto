//! # Collector of results from the doc backend

use super::*;

pub struct DocWriter<'a> {
    dest: &'a mut Vec<String>,
    buffer: String,
}

impl<'a> DocWriter<'a> {
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.buffer
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
    types: Vec<String>,
}

impl DocCollector {
    pub fn set_package_title(&mut self, title: String) {
        self.package_title = Some(title);
    }

    pub fn new_service(&mut self) -> DocWriter {
        DocWriter {
            dest: &mut self.services,
            buffer: String::new(),
        }
    }

    pub fn new_package(&mut self) -> DocWriter {
        DocWriter {
            dest: &mut self.packages,
            buffer: String::new(),
        }
    }

    pub fn new_type(&mut self) -> DocWriter {
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
            types: Vec::new(),
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
