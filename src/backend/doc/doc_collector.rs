//! # Collector of results from the doc backend

use std::fmt::Write;
use super::*;

pub struct ServiceWriter<'a> {
    collector: &'a mut DocCollector,
    buffer: String,
}

impl<'a> ServiceWriter<'a> {
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.buffer
    }
}

impl<'a> Drop for ServiceWriter<'a> {
    fn drop(&mut self) {
        self.collector.services.push(self.buffer.clone());
    }
}

pub struct PackageWriter<'a> {
    collector: &'a mut DocCollector,
    buffer: String,
}

impl<'a> PackageWriter<'a> {
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.buffer
    }
}

impl<'a> Drop for PackageWriter<'a> {
    fn drop(&mut self) {
        self.collector.package = Some(self.buffer.clone());
    }
}

pub struct DocCollector {
    buffer: String,
    package: Option<String>,
    services: Vec<String>,
}

impl DocCollector {
    pub fn new_service(&mut self) -> ServiceWriter {
        ServiceWriter {
            collector: self,
            buffer: String::new(),
        }
    }

    pub fn new_package(&mut self) -> PackageWriter {
        PackageWriter {
            collector: self,
            buffer: String::new(),
        }
    }
}

impl<'a> Collecting<'a> for DocCollector {
    type Processor = DocCompiler<'a>;

    fn new() -> Self {
        DocCollector {
            buffer: String::new(),
            package: None,
            services: Vec::new(),
        }
    }

    fn into_bytes(self, compiler: &Self::Processor) -> Result<Vec<u8>> {
        let mut out = String::new();

        compiler.processor
            .write_doc(&mut out, move |out| {
                if let Some(package) = self.package {
                    out.write_str(package.as_str())?;
                }

                for service in self.services {
                    out.write_str(service.as_str())?;
                }

                out.write_str(&self.buffer)?;
                Ok(())
            })?;

        Ok(out.into_bytes())
    }
}

impl Write for DocCollector {
    fn write_str(&mut self, other: &str) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.buffer.write_str(other)
    }
}
