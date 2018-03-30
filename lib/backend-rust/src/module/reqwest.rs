//! gRPC module for Rust.

use backend::Initializer;
use core::errors::Result;
use {Options, Service, ServiceCodegen};

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Options) -> Result<()> {
        options.service.push(Box::new(ReqwestService {}));

        Ok(())
    }
}

struct ReqwestService {}

impl ServiceCodegen for ReqwestService {
    fn generate(&self, service: Service) -> Result<()> {
        let Service { .. } = service;

        Ok(())
    }
}
