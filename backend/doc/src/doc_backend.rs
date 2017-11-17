//! Backend module for Documentation.

use backend::{Environment, PackageUtils};
use backend::errors::*;
use core::Version;
use doc_listeners::DocListeners;
use doc_options::DocOptions;
use std::collections::HashMap;

pub struct DocBackend {
    pub env: Environment,
    #[allow(dead_code)]
    options: DocOptions,
    listeners: Box<DocListeners>,
    pub theme: String,
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
    pub fn new(
        env: Environment,
        options: DocOptions,
        listeners: Box<DocListeners>,
        theme: String,
    ) -> DocBackend {
        DocBackend {
            env: env,
            options: options,
            listeners: listeners,
            theme: theme,
            themes: build_themes(),
        }
    }

    pub fn verify(&self) -> Result<()> {
        Ok(())
    }
}

impl PackageUtils for DocBackend {
    fn version_package(input: &Version) -> String {
        format!("{}", input).replace(Self::package_version_unsafe, "_")
    }
}
