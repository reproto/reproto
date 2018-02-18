//! Backend for JSON

use backend::{Environment, PackageUtils};
use core::{Handle, RpPackage};
use core::errors::*;
use json_compiler::JsonCompiler;
use json_options::JsonOptions;
use listeners::Listeners;

pub struct JsonBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
}

impl JsonBackend {
    pub fn new(env: Environment, _options: JsonOptions, listeners: Box<Listeners>) -> JsonBackend {
        JsonBackend {
            env: env,
            listeners: listeners,
        }
    }

    pub fn compiler<'el>(&'el self, handle: &'el Handle) -> Result<JsonCompiler<'el>> {
        Ok(JsonCompiler {
            handle: handle,
            processor: self,
        })
    }

    pub fn package_file(&self, package: &RpPackage) -> String {
        package.parts.join("_")
    }
}

impl PackageUtils for JsonBackend {}
