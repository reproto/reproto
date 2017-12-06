//! Module that adds fasterxml annotations to generated classes.

use backend::errors::*;
use codegen::Codegen;
use genco::{Java, Tokens};
use java_file::JavaFile;
use java_options::JavaOptions;
use listeners::Listeners;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub enum Version {
    #[serde(rename = "1")]
    Version1,
    #[serde(rename = "2")]
    Version2,
}

impl Default for Version {
    fn default() -> Self {
        Version::Version1
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    version: Version,
}

pub struct Module {
    #[allow(dead_code)]
    config: Config,
}

impl Module {
    pub fn new(config: Config) -> Module {
        Module { config: config }
    }
}

pub struct ApiClient {}

impl ApiClient {
    fn process<'el>(&self, container: &mut Tokens<'el, Java<'el>>) -> Result<()> {
        container.push("public class ApiClient {");
        container.push("}");
        Ok(())
    }
}

impl Codegen for ApiClient {
    fn generate(&self, out_path: &Path) -> Result<()> {
        JavaFile::new("io.reproto.client", "ApiClient", |out| self.process(out)).process(out_path)
    }
}

/// Build an api client.
impl ApiClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl Listeners for Module {
    fn configure(&self, options: &mut JavaOptions) -> Result<()> {
        options.root_generators.push(Box::new(ApiClient::new()));
        Ok(())
    }
}
