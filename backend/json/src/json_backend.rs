//! Backend for JSON

use backend::{CompilerOptions, Environment, PackageUtils};
use backend::errors::*;
use core::RpPackage;
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

    pub fn compiler(&self, options: CompilerOptions) -> Result<JsonCompiler> {
        Ok(JsonCompiler {
            out_path: options.out_path,
            processor: self,
        })
    }

    pub fn verify(&self) -> Result<()> {
        Ok(())
    }

    pub fn package_file(&self, package: &RpPackage) -> String {
        package.parts.join("_")
    }
}

impl PackageUtils for JsonBackend {}
